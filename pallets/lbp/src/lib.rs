#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use codec::{Decode, Encode};
use frame_support::sp_runtime::{
	app_crypto::sp_core::crypto::UncheckedFrom,
	traits::{AtLeast32BitUnsigned, BlockNumberProvider, Hash, Saturating, Zero},
	DispatchError, RuntimeDebug,
};
use frame_support::{
	dispatch::DispatchResult,
	ensure,
	traits::{EnsureOrigin, Get},
	transactional,
};
use frame_system::ensure_signed;
use hydra_dx_math::types::LBPWeight;
use orml_traits::{MultiCurrency, MultiCurrencyExtended, MultiReservableCurrency};
use primitives::traits::{AMMTransfer, AMM};
use primitives::{
	asset::AssetPair,
	constants::chain::{MAX_IN_RATIO, MAX_OUT_RATIO},
	fee::{Fee, WithFee},
	Amount, AssetId, Balance,
};

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::{marker::PhantomData, vec, vec::Vec};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod benchmarking;

#[allow(clippy::all)]
pub mod weights;
use weights::WeightInfo;
// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

type BalanceOf<T> = <<T as Config>::MultiCurrency as MultiCurrency<<T as frame_system::Config>::AccountId>>::Balance;
type PoolId<T> = <T as frame_system::Config>::AccountId;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(RuntimeDebug, Encode, Decode, Copy, Clone, PartialEq, Eq)]
pub enum WeightCurveType {
	Linear,
}

impl Default for WeightCurveType {
	fn default() -> Self {
		WeightCurveType::Linear
	}
}

/// Max weight corresponds to 100%
pub const MAX_WEIGHT: LBPWeight = 100_000_000;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(RuntimeDebug, Encode, Decode, Clone, PartialEq, Eq, Default)]
pub struct Pool<AccountId, BlockNumber: AtLeast32BitUnsigned + Copy> {
	/// owner of the pool after `CreatePoolOrigin` creates it
	pub owner: AccountId,

	/// start block
	pub start: BlockNumber,

	/// end block
	pub end: BlockNumber,

	/// Asset ids of the tokens (sold asset, accumulating asset)
	pub assets: (AssetId, AssetId),

	/// initial weight of the asset_a where the minimum value is 0 (equivalent to 0% weight), and the maximum value is 100_000_000 (equivalent to 100% weight)
	pub initial_weight: LBPWeight,

	/// final weights of the asset_a where the minimum value is 0 (equivalent to 0% weight), and the maximum value is 100_000_000 (equivalent to 100% weight)
	pub final_weight: LBPWeight,

	/// weight curve
	pub weight_curve: WeightCurveType,

	/// current fee
	pub fee: Fee,

	/// person that receives the fee
	pub fee_collector: AccountId,
}

impl<AccountId, BlockNumber: AtLeast32BitUnsigned + Copy> Pool<AccountId, BlockNumber> {
	fn new(
		pool_owner: AccountId,
		asset_a: AssetId,
		asset_b: AssetId,
		initial_weight: LBPWeight,
		final_weight: LBPWeight,
		weight_curve: WeightCurveType,
		fee: Fee,
		fee_collector: AccountId,
	) -> Self {
		Pool {
			owner: pool_owner,
			start: Zero::zero(),
			end: Zero::zero(),
			assets: (asset_a, asset_b),
			initial_weight,
			final_weight,
			weight_curve,
			fee,
			fee_collector,
		}
	}
}

pub trait LBPWeightCalculation<BlockNumber: AtLeast32BitUnsigned> {
	fn calculate_weight(
		weight_curve: WeightCurveType,
		start: BlockNumber,
		end: BlockNumber,
		initial_weight: LBPWeight,
		final_weight: LBPWeight,
		at: BlockNumber,
	) -> Option<LBPWeight>;
}

pub struct LBPWeightFunction;
impl<BlockNumber: AtLeast32BitUnsigned> LBPWeightCalculation<BlockNumber> for LBPWeightFunction {
	fn calculate_weight(
		_weight_curve: WeightCurveType,
		start: BlockNumber,
		end: BlockNumber,
		initial_weight: LBPWeight,
		final_weight: LBPWeight,
		at: BlockNumber,
	) -> Option<LBPWeight> {
		hydra_dx_math::lbp::calculate_linear_weights(start, end, initial_weight, final_weight, at).ok()
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::OriginFor;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Multi currency for transfer of currencies
		type MultiCurrency: MultiCurrencyExtended<Self::AccountId, CurrencyId = AssetId, Amount = Amount, Balance = Balance>
			+ MultiReservableCurrency<Self::AccountId>;

		#[pallet::constant]
		/// Native Asset Id
		type NativeAssetId: Get<AssetId>;

		/// The origin which can create a new pool
		type CreatePoolOrigin: EnsureOrigin<Self::Origin>;

		/// Function for calculation of LBP weights
		type LBPWeightFunction: LBPWeightCalculation<Self::BlockNumber>;

		/// Mapping of asset pairs to unique pool identities
		type AssetPairPoolId: AssetPairPoolIdFor<AssetId, PoolId<Self>>;

		/// Weight information for the extrinsics
		type WeightInfo: WeightInfo;

		/// Minimum trading limit, sole purpose of this is to keep the math working
		#[pallet::constant]
		type MinTradingLimit: Get<Balance>;

		/// Minimum pool liquidity, sole purpose of this is to keep the math working
		#[pallet::constant]
		type MinPoolLiquidity: Get<Balance>;

		/// Max fraction of pool to sell in single transaction
		#[pallet::constant]
		type MaxInRatio: Get<u128>;

		/// Max fraction of pool to buy in single transaction
		#[pallet::constant]
		type MaxOutRatio: Get<u128>;

		/// The block number provider
		type BlockNumberProvider: BlockNumberProvider<BlockNumber = Self::BlockNumber>;
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[pallet::error]
	pub enum Error<T> {
		/// Pool assets can not be the same
		CannotCreatePoolWithSameAssets,

		/// Account is not a pool owner
		NotOwner,

		/// Sale already started
		SaleStarted,

		/// Sale is still in progress
		SaleNotEnded,

		/// Sale is not running
		SaleIsNotRunning,

		/// Sale duration is too long
		MaxSaleDurationExceeded,

		/// Liquidity being added should not be zero
		CannotAddZeroLiquidity,
		/// Asset balance too low
		InsufficientAssetBalance,

		/// Pool does not exist
		PoolNotFound,

		/// Pool has been already created
		PoolAlreadyExists,

		/// Invalid block range
		InvalidBlockRange,

		/// Calculation error
		WeightCalculationError,

		/// Weight set is out of range
		InvalidWeight,

		/// Can not perform a trade with zero amount
		ZeroAmount,

		/// Trade amount is too high
		MaxInRatioExceeded,

		/// Trade amount is too high
		MaxOutRatioExceeded,

		/// Invalid fee amount
		FeeAmountInvalid,

		/// Trading limit reached
		AssetBalanceLimitExceeded,

		/// An unexpected integer overflow occurred
		Overflow, // no tests

		/// Nothing to update
		NothingToUpdate,

		/// Liquidity has not reached the required minimum.
		InsufficientLiquidity,

		/// Amount is less than minimum trading limit.
		InsufficientTradingAmount,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Pool was created by the `CreatePool` origin. [pool_id, pool_data]
		PoolCreated(PoolId<T>, Pool<T::AccountId, T::BlockNumber>),

		/// Pool data were updated. [pool_id, pool_data]
		PoolUpdated(PoolId<T>, Pool<T::AccountId, T::BlockNumber>),

		/// New liquidity was provided to the pool. [who, asset_a, asset_b, amount_a, amount_b]
		LiquidityAdded(T::AccountId, AssetId, AssetId, BalanceOf<T>, BalanceOf<T>),

		/// Liquidity was removed from the pool and the pool was destroyed. [who, asset_a, asset_b, amount_a, amount_b]
		LiquidityRemoved(T::AccountId, AssetId, AssetId, BalanceOf<T>, BalanceOf<T>),

		/// Sale executed. [who, asset_in, asset_out, amount, sale_price, fee_asset, fee_amount]
		SellExecuted(
			T::AccountId,
			AssetId,
			AssetId,
			BalanceOf<T>,
			BalanceOf<T>,
			AssetId,
			BalanceOf<T>,
		),

		/// Purchase executed. [who, asset_out, asset_in, amount, buy_price, fee_asset, fee_amount]
		BuyExecuted(
			T::AccountId,
			AssetId,
			AssetId,
			BalanceOf<T>,
			BalanceOf<T>,
			AssetId,
			BalanceOf<T>,
		),
	}

	/// Details of a pool.
	#[pallet::storage]
	#[pallet::getter(fn pool_data)]
	pub type PoolData<T: Config> =
		StorageMap<_, Blake2_128Concat, PoolId<T>, Pool<T::AccountId, T::BlockNumber>, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new liquidity bootstrapping pool for given asset pair.
		///
		/// For any asset pair, only one pool can exist at a time.
		///
		/// The dispatch origin for this call must be `T::CreatePoolOrigin`.
		/// The pool is created with initial liquidity provided by the `pool_owner` who must have
		/// sufficient funds free.
		///
		/// The pool starts uninitialized and update_pool call should be called once created to set the start block.
		///
		/// This function should be dispatched from governing entity `T::CreatePoolOrigin`
		///
		/// Parameters:
		/// - `pool_owner`: the future owner of the new pool.
		/// - `asset_a`: { asset_id, amount } Asset ID and initial liquidity amount.
		/// - `asset_b`: { asset_id, amount } Asset ID and initial liquidity amount.
		/// - `initial_weight`: Initial weight of the asset_a. 1_000_000 corresponding to 1% and 100_000_000 to 100%
		/// - `final_weight`: Final weight of the asset_a. 1_000_000 corresponding to 1% and 100_000_000 to 100%
		/// - `weight_curve`: The weight function used to update the LBP weights. Currently,
		/// there is only one weight function implemented, the linear function.
		/// - `fee`: The trading fee charged on every trade distributed to `fee_collector`.
		/// - `fee_collector`: The account to which trading fees will be transferred.
		///
		/// Emits `PoolCreated` event when successful.
		#[pallet::weight(<T as Config>::WeightInfo::create_pool())]
		#[transactional]
		pub fn create_pool(
			origin: OriginFor<T>,
			pool_owner: T::AccountId,
			asset_a: AssetId,
			asset_a_amount: Balance,
			asset_b: AssetId,
			asset_b_amount: Balance,
			initial_weight: LBPWeight,
			final_weight: LBPWeight,
			weight_curve: WeightCurveType,
			fee: Fee,
			fee_collector: T::AccountId,
		) -> DispatchResultWithPostInfo {
			T::CreatePoolOrigin::ensure_origin(origin)?;

			ensure!(
				asset_a_amount >= T::MinPoolLiquidity::get() && asset_b_amount >= T::MinPoolLiquidity::get(),
				Error::<T>::InsufficientLiquidity
			);

			ensure!(asset_a != asset_b, Error::<T>::CannotCreatePoolWithSameAssets);

			let asset_pair = AssetPair {
				asset_in: asset_a,
				asset_out: asset_b,
			};

			ensure!(!Self::exists(asset_pair), Error::<T>::PoolAlreadyExists);

			ensure!(
				T::MultiCurrency::free_balance(asset_a, &pool_owner) >= asset_a_amount,
				Error::<T>::InsufficientAssetBalance
			);

			ensure!(
				T::MultiCurrency::free_balance(asset_b, &pool_owner) >= asset_b_amount,
				Error::<T>::InsufficientAssetBalance
			);

			let pool_data = Pool::new(
				pool_owner.clone(),
				asset_a,
				asset_b,
				initial_weight,
				final_weight,
				weight_curve,
				fee,
				fee_collector,
			);

			Self::validate_pool_data(&pool_data)?;

			let pool_id = Self::get_pair_id(asset_pair);

			<PoolData<T>>::insert(&pool_id, &pool_data);

			T::MultiCurrency::transfer(asset_a, &pool_owner, &pool_id, asset_a_amount)?;
			T::MultiCurrency::transfer(asset_b, &pool_owner, &pool_id, asset_b_amount)?;

			Self::deposit_event(Event::PoolCreated(pool_id, pool_data));

			Ok(().into())
		}

		/// Update pool data of a pool.
		///
		/// The dispatch origin for this call must be signed by the pool owner.
		///
		/// The pool can be updated only if the sale has not already started.
		///
		/// At least one of the following optional parameters has to be specified.
		///
		/// Parameters:
		/// - `pool_id`: The identifier of the pool to be updated.
		/// - `start`: The new starting time of the sale. This parameter is optional.
		/// - `end`: The new ending time of the sale. This parameter is optional.
		/// - `initial_weight`: The new initial weight. This parameter is optional.
		/// - `final_weight`: The new final weight. This parameter is optional.
		/// - `fee`: The new trading fee charged on every trade. This parameter is optional.
		/// - `fee_collector`: The new receiver of trading fees. This parameter is optional.
		///
		/// Emits `PoolUpdated` event when successful.
		#[pallet::weight(<T as Config>::WeightInfo::update_pool_data())]
		#[transactional]
		pub fn update_pool_data(
			origin: OriginFor<T>,
			pool_id: PoolId<T>,
			pool_owner: Option<T::AccountId>,
			start: Option<T::BlockNumber>,
			end: Option<T::BlockNumber>,
			initial_weight: Option<LBPWeight>,
			final_weight: Option<LBPWeight>,
			fee: Option<Fee>,
			fee_collector: Option<T::AccountId>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			<PoolData<T>>::try_mutate_exists(pool_id.clone(), |maybe_pool| -> DispatchResultWithPostInfo {
				// check existence of the pool
				let mut pool = maybe_pool.as_mut().ok_or(Error::<T>::PoolNotFound)?;

				ensure!(
					start.is_some()
						|| end.is_some() || initial_weight.is_some()
						|| final_weight.is_some()
						|| fee.is_some() || fee_collector.is_some(),
					Error::<T>::NothingToUpdate
				);

				ensure!(who == pool.owner, Error::<T>::NotOwner);

				ensure!(!Self::has_pool_started(pool), Error::<T>::SaleStarted);

				pool.owner = pool_owner.unwrap_or_else(|| pool.owner.clone());

				pool.start = start.unwrap_or(pool.start);
				pool.end = end.unwrap_or(pool.end);

				pool.initial_weight = initial_weight.unwrap_or(pool.initial_weight);

				pool.final_weight = final_weight.unwrap_or(pool.final_weight);

				pool.fee = fee.unwrap_or(pool.fee);
				pool.fee_collector = fee_collector.unwrap_or_else(|| pool.fee_collector.clone());

				Self::validate_pool_data(pool)?;

				Self::deposit_event(Event::PoolUpdated(pool_id, (*pool).clone()));
				Ok(().into())
			})
		}

		/// Add liquidity to a pool.
		///
		/// Assets to add has to match the pool assets. At least one amount has to be non-zero.
		///
		/// The dispatch origin for this call must be signed by the pool owner.
		///
		/// Parameters:
		/// - `pool_id`: The identifier of the pool
		/// - `amount_a`: The identifier of the asset and the amount to add.
		/// - `amount_b`: The identifier of the second asset and the amount to add.
		///
		/// Emits `LiquidityAdded` event when successful.
		#[pallet::weight(<T as Config>::WeightInfo::add_liquidity())]
		#[transactional]
		pub fn add_liquidity(
			origin: OriginFor<T>,
			amount_a: (AssetId, BalanceOf<T>),
			amount_b: (AssetId, BalanceOf<T>),
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let (asset_a, asset_b) = (amount_a.0, amount_b.0);
			let (amount_a, amount_b) = (amount_a.1, amount_b.1);

			let pool_id = T::AssetPairPoolId::from_assets(asset_a, asset_b);
			let pool_data = <PoolData<T>>::try_get(&pool_id).map_err(|_| Error::<T>::PoolNotFound)?;

			ensure!(who == pool_data.owner, Error::<T>::NotOwner);

			ensure!(
				!amount_a.is_zero() || !amount_b.is_zero(),
				Error::<T>::CannotAddZeroLiquidity
			);

			if !amount_a.is_zero() {
				ensure!(
					T::MultiCurrency::free_balance(asset_a, &who) >= amount_a,
					Error::<T>::InsufficientAssetBalance
				);
			}

			if !amount_b.is_zero() {
				ensure!(
					T::MultiCurrency::free_balance(asset_b, &who) >= amount_b,
					Error::<T>::InsufficientAssetBalance
				);
			}

			T::MultiCurrency::transfer(asset_a, &who, &pool_id, amount_a)?;
			T::MultiCurrency::transfer(asset_b, &who, &pool_id, amount_b)?;

			Self::deposit_event(Event::LiquidityAdded(pool_id, asset_a, asset_b, amount_a, amount_b));

			Ok(().into())
		}

		/// Transfer all the liquidity from a pool back to the pool owner and destroy the pool.
		/// The pool data are also removed from the storage.
		///
		/// The pool can't be destroyed during the sale.
		///
		/// The dispatch origin for this call must be signed by the pool owner.
		///
		/// Parameters:
		/// - `amount_a`: The identifier of the asset and the amount to add.
		///
		/// Emits 'LiquidityRemoved' when successful.
		#[pallet::weight(<T as Config>::WeightInfo::remove_liquidity())]
		#[transactional]
		pub fn remove_liquidity(origin: OriginFor<T>, pool_id: PoolId<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let pool_data = <PoolData<T>>::try_get(&pool_id).map_err(|_| Error::<T>::PoolNotFound)?;

			ensure!(who == pool_data.owner, Error::<T>::NotOwner);

			ensure!(!Self::is_pool_running(&pool_data), Error::<T>::SaleNotEnded);

			let (asset_a, asset_b) = pool_data.assets;

			let amount_a = T::MultiCurrency::free_balance(asset_a, &pool_id);
			let amount_b = T::MultiCurrency::free_balance(asset_b, &pool_id);

			T::MultiCurrency::transfer(asset_a, &pool_id, &who, amount_a)?;
			T::MultiCurrency::transfer(asset_b, &pool_id, &who, amount_b)?;

			<PoolData<T>>::remove(&pool_id);

			Self::deposit_event(Event::LiquidityRemoved(pool_id, asset_a, asset_b, amount_a, amount_b));

			Ok(().into())
		}

		/// Trade `asset_in` for `asset_out`.
		///
		/// Executes a swap of `asset_in` for `asset_out`. Price is determined by the pool and is
		/// affected by the amount and proportion of the pool assets and the weights.
		///
		/// Trading `fee` is distributed to the `fee_collector`.
		///
		/// Parameters:
		/// - `asset_in`: The identifier of the asset being transferred from the account to the pool.
		/// - `asset_out`: The identifier of the asset being transferred from the pool to the account.
		/// - `amount`: The amount of `asset_in`
		/// - `max_limit`: minimum amount of `asset_out` / amount of asset_out to be obtained from the pool in exchange for `asset_in`.
		///
		/// Emits `SellExecuted` when successful.
		#[pallet::weight(<T as Config>::WeightInfo::sell())]
		#[transactional]
		pub fn sell(
			origin: OriginFor<T>,
			asset_in: AssetId,
			asset_out: AssetId,
			amount: BalanceOf<T>,
			max_limit: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			<Self as AMM<_, _, _, _>>::sell(&who, AssetPair { asset_in, asset_out }, amount, max_limit, false)?;

			Ok(().into())
		}

		/// Trade `asset_in` for `asset_out`.
		///
		/// Executes a swap of `asset_in` for `asset_out`. Price is determined by the pool and is
		/// affected by the amount and the proportion of the pool assets and the weights.
		///
		/// Trading `fee` is distributed to the `fee_collector`.
		///
		/// Parameters:
		/// - `asset_in`: The identifier of the asset being transferred from the account to the pool.
		/// - `asset_out`: The identifier of the asset being transferred from the pool to the account.
		/// - `amount`: The amount of `asset_out`.
		/// - `max_limit`: maximum amount of `asset_in` to be sold in exchange for `asset_out`.
		///
		/// Emits `BuyExecuted` when successful.
		#[pallet::weight(<T as Config>::WeightInfo::buy())]
		#[transactional]
		pub fn buy(
			origin: OriginFor<T>,
			asset_out: AssetId,
			asset_in: AssetId,
			amount: BalanceOf<T>,
			max_limit: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			<Self as AMM<_, _, _, _>>::buy(&who, AssetPair { asset_in, asset_out }, amount, max_limit, false)?;

			Ok(().into())
		}
	}
}

/// Trade type used in validation to determine how to perform certain checks
#[derive(RuntimeDebug, Clone, PartialEq, Eq)]
enum TradeType {
	Sell,
	Buy,
}

impl<T: Config> Pallet<T> {
	fn calculate_weights(
		pool_data: &Pool<T::AccountId, T::BlockNumber>,
		at: T::BlockNumber,
	) -> Result<(LBPWeight, LBPWeight), DispatchError> {
		let weight_a = T::LBPWeightFunction::calculate_weight(
			pool_data.weight_curve,
			pool_data.start,
			pool_data.end,
			pool_data.initial_weight,
			pool_data.final_weight,
			at,
		)
		.ok_or(Error::<T>::WeightCalculationError)?;

		let weight_b = MAX_WEIGHT.saturating_sub(weight_a);

		Ok((weight_a, weight_b))
	}

	fn validate_pool_data(pool_data: &Pool<T::AccountId, T::BlockNumber>) -> DispatchResult {
		let now = T::BlockNumberProvider::current_block_number();

		ensure!(
			(pool_data.start.is_zero() && pool_data.end.is_zero())
				|| (now <= pool_data.start && pool_data.start < pool_data.end),
			Error::<T>::InvalidBlockRange
		);

		// this restriction is based on the AtLeast32Bit trait of the frame_system::Balance type
		// and is expected by the calculate_linear_weights function
		ensure!(
			pool_data.end.saturating_sub(pool_data.start) <= u32::MAX.into(),
			Error::<T>::MaxSaleDurationExceeded
		);

		// zero weight at the beginning or at the end of a sale may cause a problem in the price calculation
		ensure!(
			!pool_data.initial_weight.is_zero()
				&& pool_data.initial_weight < MAX_WEIGHT
				&& !pool_data.final_weight.is_zero()
				&& pool_data.final_weight < MAX_WEIGHT,
			Error::<T>::InvalidWeight
		);

		ensure!(!pool_data.fee.denominator.is_zero(), Error::<T>::FeeAmountInvalid);

		Ok(())
	}

	fn get_sorted_weight(
		asset_in: AssetId,
		now: T::BlockNumber,
		pool_data: &Pool<T::AccountId, T::BlockNumber>,
	) -> Result<(LBPWeight, LBPWeight), Error<T>> {
		match Self::calculate_weights(pool_data, now) {
			Ok(weights) => {
				if asset_in == pool_data.assets.0 {
					Ok((weights.0, weights.1))
				} else {
					// swap weights if assets are in different order
					Ok((weights.1, weights.0))
				}
			}
			Err(_) => Err(Error::<T>::InvalidWeight),
		}
	}

	/// return true if now is in interval <pool.start, pool.end>
	fn is_pool_running(pool_data: &Pool<T::AccountId, T::BlockNumber>) -> bool {
		let now = T::BlockNumberProvider::current_block_number();
		pool_data.start <= now && now <= pool_data.end
	}

	/// return true if now is > pool.start and pool has been initialized
	fn has_pool_started(pool_data: &Pool<T::AccountId, T::BlockNumber>) -> bool {
		let now = T::BlockNumberProvider::current_block_number();
		!pool_data.start.is_zero() && pool_data.start <= now 
	}

	fn validate_trade(
		who: &T::AccountId,
		assets: AssetPair,
		amount: BalanceOf<T>,
		limit: BalanceOf<T>,
		trade_type: TradeType,
	) -> Result<AMMTransfer<T::AccountId, AssetId, AssetPair, Balance>, DispatchError> {
		ensure!(!amount.is_zero(), Error::<T>::ZeroAmount);

		if trade_type == TradeType::Sell {
			ensure!(
				T::MultiCurrency::free_balance(assets.asset_in, who) >= amount,
				Error::<T>::InsufficientAssetBalance
			);
		}

		let pool_id = Self::get_pair_id(assets);
		let pool_data = <PoolData<T>>::try_get(&pool_id).map_err(|_| Error::<T>::PoolNotFound)?;

		ensure!(Self::is_pool_running(&pool_data), Error::<T>::SaleIsNotRunning);

		let now = T::BlockNumberProvider::current_block_number();

		let (weight_in, weight_out) = Self::get_sorted_weight(assets.asset_in, now, &pool_data)?;

		let asset_in_reserve = T::MultiCurrency::free_balance(assets.asset_in, &pool_id);
		let asset_out_reserve = T::MultiCurrency::free_balance(assets.asset_out, &pool_id);

		let (amount_in, amount_out, fee_asset, fee_amount) = if trade_type == TradeType::Sell {
			ensure!(
				amount <= asset_in_reserve.checked_div(MAX_IN_RATIO).ok_or(Error::<T>::Overflow)?,
				Error::<T>::MaxInRatioExceeded
			);

			let token_amount_out = hydra_dx_math::lbp::calculate_out_given_in(
				asset_in_reserve,
				asset_out_reserve,
				weight_in,
				weight_out,
				amount,
			)
			.map_err(|_| Error::<T>::Overflow)?;

			let transfer_fee = Self::calculate_fees(&pool_data, token_amount_out)?;
			let amount_out_without_fee = token_amount_out.checked_sub(transfer_fee).ok_or(Error::<T>::Overflow)?;

			ensure!(limit <= amount_out_without_fee, Error::<T>::AssetBalanceLimitExceeded);
			(amount, amount_out_without_fee, assets.asset_out, transfer_fee)
		} else {
			ensure!(
				amount
					<= asset_out_reserve
						.checked_div(MAX_OUT_RATIO)
						.ok_or(Error::<T>::Overflow)?,
				Error::<T>::MaxOutRatioExceeded
			);

			let token_amount_in = hydra_dx_math::lbp::calculate_in_given_out(
				asset_in_reserve,
				asset_out_reserve,
				weight_in,
				weight_out,
				amount,
			)
			.map_err(|_| Error::<T>::Overflow)?;

			let transfer_fee = Self::calculate_fees(&pool_data, token_amount_in)?;
			let amount_in_with_fee = token_amount_in.checked_add(transfer_fee).ok_or(Error::<T>::Overflow)?;

			ensure!(limit >= amount_in_with_fee, Error::<T>::AssetBalanceLimitExceeded);
			(token_amount_in, amount, assets.asset_in, transfer_fee)
		};

		if trade_type == TradeType::Buy {
			ensure!(
				T::MultiCurrency::free_balance(assets.asset_in, who) >= amount_in,
				Error::<T>::InsufficientAssetBalance
			);
		}

		ensure!(
			T::MultiCurrency::free_balance(assets.asset_out, &pool_id) >= amount_out,
			Error::<T>::InsufficientAssetBalance
		);

		Ok(AMMTransfer {
			origin: who.clone(),
			assets,
			amount: amount_in,
			amount_out,
			discount: false,
			discount_amount: 0_u128,
			fee: (fee_asset, fee_amount),
		})
	}

	#[transactional]
	fn execute_trade(transfer: &AMMTransfer<T::AccountId, AssetId, AssetPair, Balance>) -> DispatchResult {
		let pool_id = Self::get_pair_id(transfer.assets);
		let pool_data = Self::pool_data(&pool_id);

		T::MultiCurrency::transfer(transfer.assets.asset_in, &transfer.origin, &pool_id, transfer.amount)?;
		T::MultiCurrency::transfer(
			transfer.assets.asset_out,
			&pool_id,
			&transfer.origin,
			transfer.amount_out,
		)?;

		if transfer.fee.0 == transfer.assets.asset_in {
			T::MultiCurrency::transfer(
				transfer.fee.0,
				&transfer.origin,
				&pool_data.fee_collector,
				transfer.fee.1,
			)?;
		} else {
			T::MultiCurrency::transfer(transfer.fee.0, &pool_id, &pool_data.fee_collector, transfer.fee.1)?;
		}

		Ok(())
	}

	fn calculate_fees(
		pool_data: &Pool<T::AccountId, T::BlockNumber>,
		amount: BalanceOf<T>,
	) -> Result<BalanceOf<T>, DispatchError> {
		Ok(amount
			.just_fee(pool_data.fee)
			.ok_or::<Error<T>>(Error::<T>::FeeAmountInvalid)?)
	}
}

pub trait AssetPairPoolIdFor<AssetId: Sized, PoolId: Sized> {
	fn from_assets(asset_a: AssetId, asset_b: AssetId) -> PoolId;
}

pub struct AssetPairPoolId<T: Config>(PhantomData<T>);

impl<T: Config> AssetPairPoolIdFor<AssetId, PoolId<T>> for AssetPairPoolId<T>
where
	PoolId<T>: UncheckedFrom<T::Hash> + AsRef<[u8]>,
{
	fn from_assets(asset_a: AssetId, asset_b: AssetId) -> PoolId<T> {
		let mut buf: Vec<u8> = b"lbp".to_vec();

		if asset_a < asset_b {
			buf.extend_from_slice(&asset_a.to_le_bytes());
			buf.extend_from_slice(&asset_b.to_le_bytes());
		} else {
			buf.extend_from_slice(&asset_b.to_le_bytes());
			buf.extend_from_slice(&asset_a.to_le_bytes());
		}
		PoolId::<T>::unchecked_from(T::Hashing::hash(&buf[..]))
	}
}

impl<T: Config> AMM<T::AccountId, AssetId, AssetPair, BalanceOf<T>> for Pallet<T> {
	fn exists(assets: AssetPair) -> bool {
		let pair_account = T::AssetPairPoolId::from_assets(assets.asset_in, assets.asset_out);
		<PoolData<T>>::contains_key(&pair_account)
	}

	fn get_pair_id(assets: AssetPair) -> T::AccountId {
		T::AssetPairPoolId::from_assets(assets.asset_in, assets.asset_out)
	}

	fn get_pool_assets(pool_account_id: &T::AccountId) -> Option<Vec<AssetId>> {
		let maybe_pool = <PoolData<T>>::try_get(pool_account_id);
		if let Ok(pool_data) = maybe_pool {
			Some(vec![pool_data.assets.0, pool_data.assets.1])
		} else {
			None
		}
	}

	/// Calculate spot price for given assets and amount. This method does not modify the storage.
	///
	/// Provided assets must exist in the pool. Panic if an asset does not exist in the pool.
	///
	/// Return 0 if calculation overflows or weights calculation overflows.
	fn get_spot_price_unchecked(asset_a: AssetId, asset_b: AssetId, amount: BalanceOf<T>) -> BalanceOf<T> {
		let pool_id = Self::get_pair_id(AssetPair {
			asset_in: asset_a,
			asset_out: asset_b,
		});

		let asset_a_reserve = T::MultiCurrency::free_balance(asset_a, &pool_id);
		let asset_b_reserve = T::MultiCurrency::free_balance(asset_b, &pool_id);

		let pool_data = match <PoolData<T>>::try_get(&pool_id) {
			Ok(pool) => pool,
			Err(_) => return BalanceOf::<T>::zero(),
		};

		let now = T::BlockNumberProvider::current_block_number();

		// We need to sort weights here if asset_in is not the first asset
		let (weight_in, weight_out) = match Self::get_sorted_weight(asset_a, now, &pool_data) {
			Ok(weights) => weights,
			Err(_) => return BalanceOf::<T>::zero(),
		};

		hydra_dx_math::lbp::calculate_spot_price(asset_a_reserve, asset_b_reserve, weight_in, weight_out, amount)
			.unwrap_or_else(|_| BalanceOf::<T>::zero())
	}

	/// Validate a sell trade and update pool weights if necessary
	fn validate_sell(
		who: &T::AccountId,
		assets: AssetPair,
		amount: BalanceOf<T>,
		min_bought: BalanceOf<T>,
		_discount: bool,
	) -> Result<AMMTransfer<T::AccountId, AssetId, AssetPair, BalanceOf<T>>, DispatchError> {
		Self::validate_trade(who, assets, amount, min_bought, TradeType::Sell)
	}

	fn execute_sell(transfer: &AMMTransfer<T::AccountId, AssetId, AssetPair, Balance>) -> DispatchResult {
		Self::execute_trade(transfer)?;

		Self::deposit_event(Event::<T>::SellExecuted(
			transfer.origin.clone(),
			transfer.assets.asset_in,
			transfer.assets.asset_out,
			transfer.amount,
			transfer.amount_out,
			transfer.fee.0,
			transfer.fee.1,
		));

		Ok(())
	}

	/// Validate buy trade and update pool weights
	fn validate_buy(
		who: &T::AccountId,
		assets: AssetPair,
		amount: BalanceOf<T>,
		max_limit: BalanceOf<T>,
		_discount: bool,
	) -> Result<AMMTransfer<T::AccountId, AssetId, AssetPair, BalanceOf<T>>, DispatchError> {
		Self::validate_trade(who, assets, amount, max_limit, TradeType::Buy)
	}

	fn execute_buy(transfer: &AMMTransfer<T::AccountId, AssetId, AssetPair, BalanceOf<T>>) -> DispatchResult {
		Self::execute_trade(transfer)?;

		Self::deposit_event(Event::<T>::BuyExecuted(
			transfer.origin.clone(),
			transfer.assets.asset_out,
			transfer.assets.asset_in,
			transfer.amount,
			transfer.amount_out,
			transfer.fee.0,
			transfer.fee.1,
		));
		Ok(())
	}

	fn get_min_trading_limit() -> Balance {
		T::MinTradingLimit::get()
	}

	fn get_min_pool_liquidity() -> Balance {
		T::MinPoolLiquidity::get()
	}

	fn get_max_in_ratio() -> u128 {
		T::MaxInRatio::get()
	}

	fn get_max_out_ratio() -> u128 {
		T::MaxOutRatio::get()
	}
}
