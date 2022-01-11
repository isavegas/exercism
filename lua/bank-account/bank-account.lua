local BankAccount = {}

function BankAccount:new()
    local account = {
        _balance = 0.0,
        _closed = false,
    }
    setmetatable(account, {__index=BankAccount})
    return account
end

function BankAccount:balance()
    return self._balance
end

function BankAccount:deposit(amount)
    if not self._closed and amount > 0 then
        self._balance = self._balance + amount
    else
        error()
    end
end

function BankAccount:withdraw(amount)
    if not self._closed and amount > 0 and self._balance > amount then
        self._balance = self._balance - amount
    else
        error()
    end
end

function BankAccount:close()
    self._closed = true
end

return BankAccount
