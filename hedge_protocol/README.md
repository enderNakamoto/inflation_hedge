## Detailed Overview

### Introduction

This protocol is a decentralized hedge solution built on the Soroban blockchain, designed to provide trustless, automated inflation protection for users. By allowing individuals and businesses to hedge against the inflation risk of volatile currencies, such as the Nigerian Naira, the platform empowers those most impacted by inflation—especially people in non-developed countries where inflation risk is high, and financial products are often inaccessible. It connects people seeking protection with investors who assume that risk in exchange for potential returns, ensuring transparent, secure, and automated transactions via smart contracts and decentralized oracles. Initially, the protocol focuses on inflation protection as a Proof of Concept.

### Problem it Solves

- Traditional financial solutions for inflation protection are often complex, costly, and inaccessible, especially for people in non-developed countries, where inflation can quickly erode savings and earnings. Many individuals in these regions lack access to stable, secure hedging options.
- Those holding volatile currencies face the severe risk of asset depreciation due to inflation, which can drastically affect day-to-day purchasing power and long-term savings.
- Markets that provide inflation protection are often out of reach or lack transparency, making it challenging for the average person to navigate them.
- The DeFi space also lacks real-world economic integration, limiting access to sustainable, non-speculative yield opportunities for investors and restricting financial inclusion.

### How it Solves These Problems

- **Decentralization and Automation:** By using Soroban smart contracts, the protocol eliminates intermediaries, reducing costs and simplifying the process, making inflation protection accessible to those who need it most.
- **Financial Inclusion:** The protocol provides a straightforward and affordable way for individuals in non-developed countries to protect their assets from inflation, offering them financial stability and empowerment.
- **Transparency:** All transactions, from hedge purchases to payouts, are conducted on-chain, allowing for public verification and eliminating trust concerns, which is especially valuable in regions with limited trust in financial institutions.
- **Timely Payouts:** Integration of Acurast Oracles enables real-time inflation data to be fed directly into the protocol, triggering automated payouts when inflation thresholds are reached, ensuring users are protected without delays.
- **Real-World Asset (RWA) Yield Source:** By bridging DeFi and real-world economic risk management, the protocol provides a stable, practical yield source, offering financial security to users and a non-speculative return option for investors.

### Audience

- **Individuals or Businesses in High-Inflation Environments:** Especially those in non-developed countries, looking for ways to protect their earnings and savings from inflationary depreciation.
- **DeFi Investors:** Investors interested in underwriting inflation risk for a stable, real-world yield.

### How It Works

- **Hedge Buyers (Protection Seekers):** Users deposit funds into a Hedge Vault to protect against inflation risk.
- **Risk Investors:** Investors deposit into a Risk Vault to act as counterparties, providing liquidity for potential payouts if inflation surpasses the target level.
- **Controller Smart Contract:** Manages fund flow between Hedge and Risk Vaults, processing payouts based on verified inflation data from Acurast Oracle.
- **Oracle Integration:** The protocol uses Acurast’s Trusted Execution Environment (TEE) to source accurate inflation data from reputable financial institutions, ensuring data reliability and transparency.

If inflation exceeds the threshold, the Risk Vault pays out to those seeking protection. If not, Hedge Vault funds transfer to the Risk Vault, offering returns to investors.

### How the Protocol Uses Soroban

- **Soroban Smart Contracts:** Soroban’s decentralized infrastructure powers the protocol, automating hedge purchases, claims, and payouts without intermediaries, making inflation protection accessible to everyone.
- **Oracle Integration:** [Acurast Oracle](https://docs.acurast.com/) supplies reliable, real-time inflation data to smart contracts, ensuring accurate execution of hedge conditions.
- **Stellar Integration:** The [`js-stellar-sdk`](https://github.com/stellar/js-stellar-sdk) facilitates seamless off-chain and on-chain data flow, essential for feeding accurate inflation data into Soroban contracts.

---

## Technical Architecture

The protocol’s decentralized architecture relies on four key actors and three main components:

### Actors
- **Hedge Buyers (Protection Seekers):** Individuals or businesses in high-inflation areas looking for inflation protection.
- **Risk Investors (Underwriters):** Those willing to assume inflation risk in exchange for potential returns.
- **Decentralized Liquidators (Incentivized Bots):** Agents responsible for enforcing liquidations.
- **Market Makers:** Users who create and launch various Hedge/Risk Markets.

### Main Components
- **Smart Contracts:** Immutable contracts on Soroban that facilitate hedge transactions and manage payouts.
- **Frontend:** A web-based interface to provide easy access for all users, regardless of technical expertise.
- **Oracle:** A Node.js script on [Acurast Processors](https://docs.acurast.com/acurast-processors) to fetch inflation data.

### Protocol’s Vault System

Users deposit liquidity in **Hedge Vaults** (for inflation protection) or **Risk Vaults** (as counterparties) to manage capital based on their preferred risk positions. The protocol issues LP tokens to represent users' shares in these vaults, ensuring traceability of ownership.

Using an **Oracle** script on an **Acurast TEE Processor**, the protocol retrieves inflation data from trusted sources (such as central banks) and integrates it with Soroban smart contracts through [`js-stellar-sdk`](https://github.com/stellar/js-stellar-sdk).

The controller contract then assesses inflation data to determine if payout conditions are met. If inflation surpasses the threshold, funds move from the Risk Vault to the Hedge Vault for payouts; if not, funds in the Hedge Vault are transferred to the Risk Vault, providing returns to investors.
