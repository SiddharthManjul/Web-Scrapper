# VestAO
VestAO is an asset streaming and distribution protocol bringing subscriptions, salaries, vesting, and rewards to DAOs and crypto-native businesses worldwide. This is made possible by the protocol’s smart contract framework which introduces the Streaming Token - an extension to the basic AO Token standard enabling the transfer of value in time-based manner.
VestAO leverages blockchain technology and smart contracts to automate and manage the vesting of assets, such as tokens, for participants in various financial programs. This type of platform is particularly useful for DAOs (Decentralized Autonomous Organizations), crypto-native businesses, and projects that distribute tokens or other digital assets to contributors, investors, and employees.

## Key Features
### Smart Contract Automation: 
VestAO runs Smart Contracts which ensure that vesting schedules and conditions are immutable and automatically enforced without the need for intermediaries. All transactions and vesting schedules are recorded on the blockchain providing transparency to all stakeholders.
### Customizable Vesting Schedules:
It supports user-based scheduling which means that they can create their own scheduling, can start it anytime, can schedule it for future. Vesting conditions can be tied to specific milestones or performance metrics.
### Token Management:
Utilizes advanced token standards, such as Streaming Tokens, which extend basic AO Token standards, to facilitate continuous and dynamic asset streaming. Tokens are automatically distributed to beneficiaries according to the vesting schedule, reducing administrative overhead.
### Employee Compensation:
Automates the vesting of employee stock options, ensuring that employees receive their tokens according to predefined schedules. Manages various employee incentive programs, such as bonuses and performance-based rewards.
### Investor Vesting:
Manages the vesting of tokens for seed investors, ensuring that tokens are released gradually to prevent market dumping. Distributes tokens to crowdsale participants in a controlled manner, adhering to the terms of the sale.

## Technical Aspects:
### Basic Token Specs:
Since the Streaming Tokens are extended version of basic AO Tokens. It operates all the specs of AO Tokens which are "Info", "Balance", "Balances", "Transfer", and "Mint" in form of AO Hanlders.
#### Info Handler:
Info Handler provides the information about the Token. 

```Lua
Handlers.add('info', Handlers.utils.hasMatchingTag('Action', 'Info'), function(msg)
  ao.send(
      { Target = msg.From, Tags = { Name = Name, Ticker = Ticker, Logo = Logo, Denomination = tostring(Denomination) } })
end)
```
This code means that if someone Sends a message with the Tag, Action = "info", our token will Send back a message with all of the information defined above. Note the Target = msg.From, this tells ao we are replying to the process that sent us this message.
#### Balance Handler:
Balance Handler provides the token balance someone holds,

```Lua
Handlers.add('balance', Handlers.utils.hasMatchingTag('Action', 'Balance'), function(msg)
  local bal = '0'

  -- If not Target is provided, then return the Senders balance
  if (msg.Tags.Target and Balances[msg.Tags.Target]) then
    bal = tostring(Balances[msg.Tags.Target])
  elseif Balances[msg.From] then
    bal = tostring(Balances[msg.From])
  end

  ao.send({
    Target = msg.From,
    Tags = { Target = msg.From, Balance = bal, Ticker = Ticker, Data = json.encode(tonumber(bal)) }
  })
end)
```
The first Handler above Handlers.add('balance' handles a process or person requesting their own balance or the balance of a Target. Then replies with a message containing the info.
#### Balances Handler:
Balances Handler provides the entire Balance Table

```Lua
Handlers.add('balances', Handlers.utils.hasMatchingTag('Action', 'Balances'),
             function(msg) ao.send({ Target = msg.From, Data = json.encode(Balances) }) end)
```
