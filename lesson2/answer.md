# POE pallet

## Node running

![run](assets/run.png)

## Create claim

### Success

![create](assets/create.png)

![check_create](assets/check_create.png)

### Fail 

**Bonus: hash should not longer than two u8**

![create_fail](assets/create_fail.png)

## Transfer claim

### Fail

#### Claim not created

![transfer_fail](assets/transfer_fail.png)

#### Sender is not the owner

![transfer_fail](assets/transfer_fail_2.png)

### Success

![transfer](assets/transfer.png)

![check_transfer](assets/check_transfer.png)

## Revoke claim

### Fail

#### Origin is not the owner

![revoke_fail](assets/revoke_fail.png)

#### Claim not created

![revoke_fail](assets/revoke_fail_2.png)

### Success

![revoke](assets/revoke.png)

![check_revoke](assets/check_revoke.png)
