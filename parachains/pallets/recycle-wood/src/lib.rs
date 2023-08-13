//! Pallet to spam the XCM/UMP.

#![cfg_attr(not(feature = "std"), no_std)]

use cumulus_pallet_xcm::{ensure_sibling_para, Origin as CumulusOrigin};
use cumulus_primitives_core::ParaId;
use frame_support::{parameter_types, BoundedVec};
use frame_system::Config as SystemConfig;
use sp_std::prelude::*;
use xcm::latest::prelude::*;

pub use pallet::*;

parameter_types! {
	const MaxParachains: u32 = 100;
	
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::{sp_runtime::traits::{Hash},
                        traits::{ Randomness}};

	use frame_support::sp_io::hashing::blake2_128;
	use frame_support::traits::Time;

	#[derive(Clone, Encode, Decode, RuntimeDebug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct WoodProduct<T: Config> {
		pub dna: [u8; 16],   // Using 16 bytes to represent a ID of the Product
		pub name: BoundedVec<u8, T::MaxPayloadSize>,
		pub description : BoundedVec<u8, T::MaxPayloadSize>,
		pub ownerpallet :ParaId,
		pub previouspallet :ParaId,
		pub createdtime :MomentOf<T>,
	}


	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type RuntimeOrigin: From<<Self as SystemConfig>::RuntimeOrigin>
			+ Into<Result<CumulusOrigin, <Self as Config>::RuntimeOrigin>>;

		/// The overarching call type; we assume sibling chains use the same type.
		type RuntimeCall: From<Call<Self>> + Encode;

		type XcmSender: SendXcm;

		#[pallet::constant]
		type MaxPayloadSize: Get<u32>;
		type MaxProductOwned: Get<u32>;

		type WoodProductRandomness: Randomness<Self::Hash, Self::BlockNumber>;
		type Time: Time;
	}

	type MomentOf<T> = <<T as Config>::Time as Time>::Moment;
	

	/// The total number of products registered.
	#[pallet::storage]
	#[pallet::getter(fn all_products_count)]
	pub(super) type AllProductsCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn para_owned_products)]
	pub(super) type ProductOwnedByParachains<T: Config> = StorageMap<
    	_, 
    	Twox64Concat, 
    	ParaId, 
    	BoundedVec<T::Hash, T::MaxProductOwned>,
    	ValueQuery
    	>;

	#[pallet::storage]
	#[pallet::getter(fn products_owned)] 
	pub(super) type ProductOwned<T: Config> =  StorageValue<_,BoundedVec<T::Hash, T::MaxProductOwned>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn wood_products)]
	/// Stores wood product.
	pub(super) type WoodProducts<T: Config> = StorageMap<_, Twox64Concat, T::Hash, WoodProduct<T>,OptionQuery>;


	#[pallet::storage]
	#[pallet::getter(fn transfer_requests)]
	/// Stores wood product.
	pub(super) type TransferRequests<T: Config> = StorageMap<_, Twox64Concat, T::Hash, ParaId,OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn request_list)] 
	pub(super) type RequestList<T: Config> =  StorageValue<_,BoundedVec<T::Hash, T::MaxProductOwned>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		/// A new Product was sucessfully created. \[sender, product_id\]
		ProductCreateSent(ParaId,Vec<u8>, XcmHash, MultiAssets ),

		/// A Product was sucessfully transferred. \[from, to, product_id\]
		Transferred(T::AccountId, T::AccountId, T::Hash),
		/// A Product was sucessfully bought. \[buyer, seller, kitty_id, bid_price\]
		Bought(T::AccountId, T::AccountId, T::Hash),

		ProductMinted(T::Hash),


		ErrorCreatingProduct(SendError, ParaId, Vec<u8>),

		MintReceived(ParaId, Vec<u8>),
		ErrorSendingProductId(SendError, ParaId, T::Hash),
		ProductIdSent(T::Hash),

		AddProductRecieved(ParaId,T::Hash),

		ProductAdded(ParaId,T::Hash),

		ForwardTransferRequestSent(ParaId,T::Hash),

		ErrorSendingForwardTransferRequest(SendError,ParaId,T::Hash),

		ErrorSendingTransferRequest(SendError,ParaId,T::Hash),

		ErrorSendingTransferRejectRequest(SendError,ParaId,T::Hash),

		ErrorSendingTransferNotificationRequest(SendError,ParaId,T::Hash,bool),

		TransferRejectedSuccessfully(T::Hash),

		ProductTransferCompletedAddedToParachain(T::Hash),

		ProductTransferRejected(T::Hash),

		TransferCompleted(T::Hash,bool),

		TransferNotificationSent(T::Hash,bool),



		ForwardedTransferRequest(ParaId,T::Hash),

		TransferRequestAdded(ParaId,T::Hash),
		TransferRequestSent(ParaId,T::Hash),
		ErrorSendingTransferApprovalRequest(SendError,ParaId,T::Hash),
		TransferApprovalSent(T::Hash),

	}

	#[pallet::error]
	pub enum Error<T> {

		/// Handles arithemtic overflow when incrementing the Kitty counter.
		ProductCntOverflow,
		/// An account cannot own more Kitties than `MaxKittyCount`.
		ExceedMaxProductOwned,
		/// Buyer cannot be the owner.
		BuyerIsProductOwner,
		/// Cannot transfer a kitty to its owner.
		TransferToSelf,
		/// Handles checking whether the Kitty exists.
		ProductNotExist,
		/// Handles checking that the Kitty is owned by the account transferring, buying or setting a price for it.
		NotProductOwner,
		/// Ensures the Kitty is for sale.
		ProductNotForSale,
		/// Ensures that the buying price is greater than the asking price.
		ProductBidPriceTooLow,
		/// Ensures that an account has enough funds to purchase a Kitty.
		NotEnoughBalance,

		NameTooLarge,

		DescriptionTooLarge,
		
		NoTransferToSelf,

		RequestNotExist,

		
	}

	//String s1 = "something";
	//String s2 =s1; 
	//sysout(s1);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		#[pallet::call_index(0)]
		#[pallet::weight({0})]
		pub fn create_wood_product(origin: OriginFor<T>, name: Vec<u8>,  description: Vec<u8>) -> DispatchResult {
			ensure_root(origin.clone())?;
			let namecopy= name.clone();
			let para = ParaId::from(1001);
			let descriptioncopy = description.clone();
			//let sender = ensure_signed(origin.clone())?;
			match send_xcm::<T::XcmSender>(
				(Parent, Junction::Parachain(para.into())).into(),
				Xcm(vec![Transact {
					origin_kind: OriginKind::Native,
					require_weight_at_most: Weight::from_parts(1_000, 1_000),
					call: <T as Config>::RuntimeCall::from(Call::<T>::mint {
						name:namecopy.clone(),
						description:descriptioncopy,
					})
					.encode()
					.into(),
				}]),
			) {
				Ok((hash, cost)) => {
					Self::deposit_event(Event::ProductCreateSent(
						para,
						namecopy.clone(),
						hash,
						cost,
					));
				},
				Err(e) => {
					Self::deposit_event(Event::ErrorCreatingProduct(
						e,
						para,
						namecopy.clone()
					));
				},
			}
			Ok(())
			
		}
		#[pallet::call_index(1)]
		#[pallet::weight({0})]
		pub fn mint(
			owner: OriginFor<T>,
			name: Vec<u8>,
			description : Vec<u8>,
			) -> DispatchResult  {
				let paraid = ensure_sibling_para(<T as Config>::RuntimeOrigin::from(owner.clone()))?;
				Self::deposit_event(Event::MintReceived(paraid, name.clone()));
				let boundedname = BoundedVec::<u8, T::MaxPayloadSize>::try_from(name)
				.map_err(|_| Error::<T>::NameTooLarge)?;
				let boundeddescription = BoundedVec::<u8, T::MaxPayloadSize>::try_from(description)
				.map_err(|_| Error::<T>::DescriptionTooLarge)?;

				let dna = None;
				let current_time = T::Time::now();
				
				let product = WoodProduct::<T> {
				  dna: dna.unwrap_or_else(Self::gen_dna),
				  name: boundedname,
				  description: boundeddescription,
				  ownerpallet : paraid.clone(),
				  previouspallet: paraid.clone(),
				  createdtime: current_time,

				};
		  
			let product_id = T::Hashing::hash_of(&product);


			match send_xcm::<T::XcmSender>(
				(Parent, Junction::Parachain(paraid.into())).into(),
				Xcm(vec![Transact {
					origin_kind: OriginKind::Native,
					require_weight_at_most: Weight::from_parts(1_000, 1_000),
					call: <T as Config>::RuntimeCall::from(Call::<T>::add_product_hash {
						product_id:product_id.clone(),
					})
					.encode()
					.into(),
				}]),
			) {
				Ok((_, _)) => {
					Self::deposit_event(Event::ProductIdSent(
						product_id.clone()
					));
					let new_cnt = Self::all_products_count().checked_add(1)
					.ok_or(<Error<T>>::ProductCntOverflow)?;
	
				<ProductOwnedByParachains<T>>::try_mutate(paraid.clone(),|product_vec| {
					product_vec.try_push(product_id)
				}).map_err(|_| <Error<T>>::ExceedMaxProductOwned)?;
			  
				<WoodProducts<T>>::insert(product_id, product);
				<AllProductsCount<T>>::put(new_cnt);
				},
				Err(e) => {
					Self::deposit_event(Event::ErrorSendingProductId(
						e,
						paraid,
						product_id.clone()
					));
				},
			}
			Self::deposit_event(Event::ProductMinted(product_id.clone()));
			Ok(())
		}

		
		#[pallet::call_index(2)]
		#[pallet::weight({0})]
		pub fn forward_transfer_request(
			origin: OriginFor<T>, 
			product_id: T::Hash
		) -> DispatchResult {
			let paraid = ensure_sibling_para(<T as Config>::RuntimeOrigin::from(origin.clone()))?;

			ensure!(Self::is_product_owner(&product_id,&paraid)?, <Error<T>>::NoTransferToSelf);

			let product = Self::wood_products(&product_id).ok_or(<Error<T>>::ProductNotExist)?;
			let owner_para_id = product.ownerpallet.clone();
			match send_xcm::<T::XcmSender>(
				(Parent, Junction::Parachain(owner_para_id.into())).into(),
				Xcm(vec![Transact {
					origin_kind: OriginKind::Native,
					require_weight_at_most: Weight::from_parts(1_000, 1_000),
					call: <T as Config>::RuntimeCall::from(Call::<T>::receive_transfer_request {
						product_id:product_id.clone(),
					})
					.encode()
					.into(),
				}]),
			) {
				Ok((_, _)) => {
					Self::deposit_event(Event::ForwardTransferRequestSent(
						owner_para_id,
						product_id.clone()
					));
			  
				<TransferRequests<T>>::insert(product_id, paraid);
				},
				Err(e) => {
					Self::deposit_event(Event::ErrorSendingForwardTransferRequest(
						e,
						owner_para_id,
						product_id.clone()
					));
				},
			}
		
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight({0})]
		pub fn add_product_hash(origin: OriginFor<T>,product_id: T::Hash)-> DispatchResult  {
			let paraid = ensure_sibling_para(<T as Config>::RuntimeOrigin::from(origin.clone()))?;
			
			Self::deposit_event(Event::AddProductRecieved(paraid, product_id.clone()));

			<ProductOwned<T>>::try_append(product_id).map_err(|_| <Error<T>>::ExceedMaxProductOwned)?;
			Self::deposit_event(Event::ProductAdded(paraid, product_id.clone()));

			Ok(())
		}
		#[pallet::call_index(4)]
		#[pallet::weight({0})]
		pub fn receive_transfer_request(
			origin: OriginFor<T>, 
			product_id: T::Hash
		) -> DispatchResult {
			let paraid = ensure_sibling_para(<T as Config>::RuntimeOrigin::from(origin.clone()))?;
			<RequestList<T>>::try_append(product_id).map_err(|_| <Error<T>>::ExceedMaxProductOwned)?;
			Self::deposit_event(Event::TransferRequestAdded(paraid, product_id.clone()));
			Ok(())
		}


		#[pallet::call_index(5)]
		#[pallet::weight({0})]
		pub fn transfer_request(
			origin: OriginFor<T>, 
			product_id: T::Hash
		) -> DispatchResult {
			ensure_root(origin.clone())?;
			let paraid = ParaId::from(1001);
			<RequestList<T>>::try_append(product_id).map_err(|_| <Error<T>>::ExceedMaxProductOwned)?;
			match send_xcm::<T::XcmSender>(
				(Parent, Junction::Parachain(paraid.into())).into(),
				Xcm(vec![Transact {
					origin_kind: OriginKind::Native,
					require_weight_at_most: Weight::from_parts(1_000, 1_000),
					call: <T as Config>::RuntimeCall::from(Call::<T>::forward_transfer_request {
						product_id:product_id.clone(),
					})
					.encode()
					.into(),
				}]),
			) {
				Ok((_, _)) => {
					Self::deposit_event(Event::TransferRequestSent(
						paraid,
						product_id.clone()
					));
			  
				<TransferRequests<T>>::insert(product_id, paraid);
				},
				Err(e) => {
					Self::deposit_event(Event::ErrorSendingTransferRequest(
						e,
						paraid,
						product_id.clone()
					));
				},
			}
			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight({0})]
		pub fn approve_transfer_request(
			origin: OriginFor<T>, 
			product_id: T::Hash
		) -> DispatchResult {
			ensure_root(origin.clone())?;

			let paraid = ParaId::from(1001);
			ProductOwned::<T>::try_mutate(|v1| {
				if let Some(index1) = v1.iter().position(|&	x1| x1 == product_id) {
					v1.swap_remove(index1);
					return Ok(());
				}else{
					Err(())
				}
			}).map_err(|_| <Error<T>>::ExceedMaxProductOwned)?;

			RequestList::<T>::try_mutate(|v| {
				if let Some(index) = v.iter().position(|&	x| x == product_id) {
					//code changes.
					match send_xcm::<T::XcmSender>(
						(Parent, Junction::Parachain(paraid.into())).into(),
						Xcm(vec![Transact {
							origin_kind: OriginKind::Native,
							require_weight_at_most: Weight::from_parts(1_000, 1_000),
							call: <T as Config>::RuntimeCall::from(Call::<T>::complete_transfer {
								product_id:product_id.clone(),
								approval:true
							})
							.encode()
							.into(),
						}]),
					) {
						Ok((_, _)) => {
							Self::deposit_event(Event::TransferApprovalSent(
								product_id.clone()
							));
							v.swap_remove(index);
						},
						Err(e) => {
							Self::deposit_event(Event::ErrorSendingTransferApprovalRequest(
								e,
								paraid,
								product_id.clone()
							));

						},
					};
					return Ok(());
				}else {
					Err(())
				}
			}).map_err(|_| <Error<T>>::ExceedMaxProductOwned)?;

			Ok(())
		}

		#[pallet::call_index(7)]
		#[pallet::weight({0})]
		pub fn reject_transfer_request(
			origin: OriginFor<T>, 
			product_id: T::Hash
		) -> DispatchResult {
			ensure_root(origin.clone())?;

			let paraid = ParaId::from(1001);

			RequestList::<T>::try_mutate(|v| {
				if let Some(index) = v.iter().position(|&	x| x == product_id) {
					//code changes.
					match send_xcm::<T::XcmSender>(
						(Parent, Junction::Parachain(paraid.into())).into(),
						Xcm(vec![Transact {
							origin_kind: OriginKind::Native,
							require_weight_at_most: Weight::from_parts(1_000, 1_000),
							call: <T as Config>::RuntimeCall::from(Call::<T>::complete_transfer {
								product_id:product_id.clone(),
								approval:false
							})
							.encode()
							.into(),
						}]),
					) {
						Ok((_, _)) => {
							Self::deposit_event(Event::TransferRejectedSuccessfully(
								product_id.clone()
							));
							v.swap_remove(index);
						},
						Err(e) => {
							Self::deposit_event(Event::ErrorSendingTransferRejectRequest(
								e,
								paraid,
								product_id.clone()
							));
						},
					};
					return Ok(());
				}else {
					Err(())
				}
			}).map_err(|_| <Error<T>>::ExceedMaxProductOwned)?;

			Ok(())
		}



		#[pallet::call_index(8)]
		#[pallet::weight({0})]
		pub fn complete_transfer(
			origin: OriginFor<T>, 
			product_id: T::Hash,
			approval: bool
		) -> DispatchResult {
			let paraid = ensure_sibling_para(<T as Config>::RuntimeOrigin::from(origin.clone()))?;
			let transfer_request_para_id = Self::transfer_requests(&product_id).ok_or(<Error<T>>::ProductNotExist)?;
			
			if approval{
				let mut product = Self::wood_products(&product_id).ok_or(<Error<T>>::ProductNotExist)?;
				let prev_owner = product.ownerpallet.clone();
				<ProductOwnedByParachains<T>>::try_mutate(&prev_owner, |products| {
					if let Some(ind) = products.iter().position(|&id| id == product_id) {
						products.swap_remove(ind);
						return Ok(());
					}
					Err(())
				}).map_err(|_| <Error<T>>::ProductNotExist)?;


				product.ownerpallet = transfer_request_para_id;
				product.previouspallet = prev_owner;
				<WoodProducts<T>>::insert(product_id, product);
				
			}
			<TransferRequests<T>>::remove(product_id);

			match send_xcm::<T::XcmSender>(
				(Parent, Junction::Parachain(transfer_request_para_id.into())).into(),
				Xcm(vec![Transact {
					origin_kind: OriginKind::Native,
					require_weight_at_most: Weight::from_parts(1_000, 1_000),
					call: <T as Config>::RuntimeCall::from(Call::<T>::transfer_notification {
						product_id:product_id.clone(),
						approval:approval
					})
					.encode()
					.into(),
				}]),
			) {
				Ok((_, _)) => {
					Self::deposit_event(Event::TransferNotificationSent(
						product_id.clone(),
						approval
					));
				},
				Err(e) => {
					Self::deposit_event(Event::ErrorSendingTransferNotificationRequest(
						e,
						paraid,
						product_id.clone(),
						approval
					));
				},
			};

			Self::deposit_event(Event::TransferCompleted(
				product_id.clone(),
				approval
			));

			Ok(())
		}

		#[pallet::call_index(9)]
		#[pallet::weight({0})]
		pub fn transfer_notification(
			_: OriginFor<T>, 
			product_id: T::Hash,
			approval: bool
		) -> DispatchResult {


			if approval{
				<ProductOwned<T>>::try_append(product_id.clone()).map_err(|_| <Error<T>>::ExceedMaxProductOwned)?;
				Self::deposit_event(Event::ProductTransferCompletedAddedToParachain(
					product_id.clone(),
				));
			}
			else{
				Self::deposit_event(Event::ProductTransferRejected(
					product_id.clone(),
				));
			}
			Ok(())
		}
	}


	

	impl<T: Config> Pallet<T> {
		pub fn gen_dna() -> [u8; 16] {
			let payload = (
				T::WoodProductRandomness::random(&b"dna"[..]).0,
				<frame_system::Pallet<T>>::block_number(),
			);
			payload.using_encoded(blake2_128)
		}

		pub fn is_product_owner(product_id: &T::Hash, paraid: &ParaId) -> Result<bool, Error<T>> {
			match Self::wood_products(product_id) {
			  Some(product) => Ok(product.ownerpallet != *paraid),
			  None => Err(<Error<T>>::ProductNotExist)
			}

		}


	}
}
