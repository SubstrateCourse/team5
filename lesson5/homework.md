# hw5

## 补完剩下的代码

## 设计如何实现 transfer kitty 赠予小猫这个功能

1. 从拥有者处将小猫删除；
2. 为接受者创建对应的小猫；

### 分析是否需要修改或者新增 storage 数据结构，如何修改

可以不修改

### 使用伪代码解释流程

完整代码参见 lib.rs

```rust
	pub fn transfer(origin, user_kitty_id: u32, to: <T::Lookup as StaticLookup>::Source) {
		let sender = ensure_signed(origin)?;

		let from_user_kitties_count = OwnedKittiesCount::<T>::get(&sender);
		ensure!(from_user_kitties_count > user_kitty_id, Error::<T>::UserNotHaveTheKitty);

		// check to user kitties will not flow
		let to = T::Lookup::lookup(to)?;
		let to_user_kitties_count = OwnedKittiesCount::<T>::get(&to);
		if to_user_kitties_count == u32::max_value() {
			return Err(Error::<T>::KittiesCountOverflow.into());
		}

		// remove the from user kitty
		let kitty_id = OwnedKitties::<T>::get((&sender, user_kitty_id));
		OwnedKittiesCount::<T>::insert(&sender, from_user_kitties_count - 1);
		if user_kitty_id + 1 != from_user_kitties_count {
			// move the last user kitty to the removed position
			let from_last_kitty_id = OwnedKitties::<T>::get((&sender, from_user_kitties_count - 1));
			OwnedKitties::<T>::remove((&sender, from_user_kitties_count - 1));
			OwnedKitties::<T>::insert((&sender, user_kitty_id), from_last_kitty_id);
		} else {
			OwnedKitties::<T>::remove((&sender, user_kitty_id));
		}

		// add the to user kitty
		OwnedKittiesCount::<T>::insert(&to, to_user_kitties_count + 1);
		OwnedKitties::<T>::insert((&to, to_user_kitties_count + 1), kitty_id);
	}
}
```

### 加分项：复杂度优于 O(n), n = 用户拥有的小猫的数量

## 设计如何实现简单的交易功能，用户可以给自己小猫定价，然后其他人可以出钱购买

1. 新增数据结构，记录小猫价格；
2. 新增接口，设置小猫价格；
3. 新增接口，购买小猫，如果价格不存在，则不能购买，如果出价高于设置的价格则触发小猫的转移，否则购买失败。购买成功后清除价格。

### 分析新增的 storage 数据结构

```
pub Price get(fn price): map hasher(blake2_128_concat) u32 => BalanceOf<T>;
pub KittyOwner get(kitty_owner): map hasher(blake_128_concat) u32 => T::AccountId;
```

Price 的 key 为 kitty_id, value 为价格。类似 lesson3 的 claim price。

KittyOwner 记录 kitty_id 到 account 的映射。

### 使用伪代码解释流程

流程类似 lesson 3 的 set_price 和 buy_claim.

```
fn set_price(origin, user_kitty_id, price: BalanceOf<T>) {
    ensure_signed;
    ensure_user_have_the_kitty;
    let kitty_id = query_user_kitty(sender, user_kitty_id);
    Price::insert(kitty_id, price);
}

fn buy_kitty(origin, kitty_id: u32) {
    ensure_signed;
    
    check_kitty_exists;
    check_kitty_for_sell;
    
    let price = Price::get(kitty_id);
    let owner = KittyOwner::get(kitty_id);
    check_balance_enough;
    check_owner_is_not_self;
    check_kitty_will_not_overflow;
    
    transfer(owner, price);
    transfer_kitty(owner, sender, kitty_id);
}
```
