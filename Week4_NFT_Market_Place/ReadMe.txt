NFT Swap Marketplace

A Trustless, Two-Sided NFT-for-NFT Swap Platform on Solana

This project implements a decentralized NFT swap marketplace that enables users to exchange NFTs directly with one another through a fully on-chain escrow mechanism.
Built using Solana, Anchor 0.31.1, and program-owned PDAs, the system ensures secure, atomic NFT exchanges without requiring trust between participants.

Features
Peer-to-Peer NFT Swaps

Enables users to trade NFTs directly, without needing to convert to tokens or rely on centralized platforms.

Secure Escrow Vaults (PDAs)

Both the seller’s and buyer’s NFTs are held in program-owned escrow accounts, preventing unauthorized access or double spending.

Atomic Swap Execution

When the seller accepts an offer, the program performs an atomic swap in a single transaction:

Seller’s NFT is transferred to the buyer

Buyer’s NFT is transferred to the seller

Escrow accounts are closed automatically

This ensures that no partial or incomplete swaps are possible.

Two-Sided Marketplace Logic

Sellers list their NFTs for swap

Buyers make offers using their own NFTs

Cancellable Actions

Sellers may cancel listings at any time

Buyers may cancel offers at any time
NFTs are immediately returned to their respective owners.



How It Works

1. Seller Creates a Listing

	The seller selects an NFT

	The NFT is transferred into a Seller Vault PDA

	A Listing PDA is created to store listing metadata

2. Buyer Submits an Offer

	The buyer selects an NFT to offer

	The NFT is transferred into a Buyer Vault PDA

	An Offer PDA is created

3. Seller Accepts an Offer

	The program executes an atomic swap:

SellerVaultPDA  -> Buyer’s ATA
BuyerVaultPDA   -> Seller’s ATA


Then:

Listing PDA is closed

Offer PDA is closed

Vault ATAs are closed

Lamports are refunded to owners

4. Cancellation Logic

A seller may cancel the listing to retrieve their NFT

A buyer may cancel an offer to retrieve their NFT

All cancellations release locked NFTs safely.

Security Design

PDA ownership enforcement ensures only the program can move escrowed assets

Mint and metadata verification protects against spoofed or invalid NFTs

Seed constraints prevent reinitialization attacks

Locked NFTs cannot be withdrawn until swap or cancellation

Atomic transactions eliminate the possibility of partial execution

Account closures prevent rent waste and lingering storage
