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
#### Transfer Handler:
This Handler performs the Transfer Action to transfer token from sender to receiver.
```Lua
Handlers.add('transfer', Handlers.utils.hasMatchingTag('Action', 'Transfer'), function(msg)
  assert(type(msg.Tags.Recipient) == 'string', 'Recipient is required!')
  assert(type(msg.Tags.Quantity) == 'string', 'Quantity is required!')

  if not Balances[msg.From] then Balances[msg.From] = 0 end

  if not Balances[msg.Tags.Recipient] then Balances[msg.Tags.Recipient] = 0 end

  local qty = tonumber(msg.Tags.Quantity)
  assert(type(qty) == 'number', 'qty must be number')

  if Balances[msg.From] >= qty then
    Balances[msg.From] = Balances[msg.From] - qty
    Balances[msg.Tags.Recipient] = Balances[msg.Tags.Recipient] + qty

    --[[
      Only Send the notifications to the Sender and Recipient
      if the Cast tag is not set on the Transfer message
    ]] --
    if not msg.Tags.Cast then
      -- Debit-Notice message template, that is sent to the Sender of the transfer
      local debitNotice = {
        Target = msg.From,
        Action = 'Debit-Notice',
        Recipient = msg.Recipient,
        Quantity = tostring(qty),
        Data = Colors.gray ..
            "You transferred " ..
            Colors.blue .. msg.Quantity .. Colors.gray .. " to " .. Colors.green .. msg.Recipient .. Colors.reset
      }
      -- Credit-Notice message template, that is sent to the Recipient of the transfer
      local creditNotice = {
        Target = msg.Recipient,
        Action = 'Credit-Notice',
        Sender = msg.From,
        Quantity = tostring(qty),
        Data = Colors.gray ..
            "You received " ..
            Colors.blue .. msg.Quantity .. Colors.gray .. " from " .. Colors.green .. msg.From .. Colors.reset
      }

      -- Add forwarded tags to the credit and debit notice messages
      for tagName, tagValue in pairs(msg) do
        -- Tags beginning with "X-" are forwarded
        if string.sub(tagName, 1, 2) == "X-" then
          debitNotice[tagName] = tagValue
          creditNotice[tagName] = tagValue
        end
      end

      -- Send Debit-Notice and Credit-Notice
      ao.send(debitNotice)
      ao.send(creditNotice)
    end
  else
    ao.send({
      Target = msg.Tags.From,
      Tags = { Action = 'Transfer-Error', ['Message-Id'] = msg.Id, Error = 'Insufficient Balance!' }
    })
  end
end)
```
In summary, this code checks to make sure the Recipient and Quantity Tags have been provided, initializes the balances of the person sending the message and the Recipient if they dont exist and then attempts to transfer the specified quantity to the Recipient in the Balances table. If the transfer was successful a Debit-Notice is sent to the sender of the original message and a Credit-Notice is sent to the Recipient. If there was insufficient balance for the transfer it sends back a failure message. The line if not msg.Tags.Cast then Means were not producing any messages to push if the Cast tag was set. This is part of the ao protocol.
