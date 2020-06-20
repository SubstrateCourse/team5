# Homework

## 1. 补完剩下的代码

## 2. 设计如何实现 transfer kitty 赠予小猫这个功能

### a. 分析是否需要修改或者新增 storage 数据结构，如何修改

使用链表存储用户所有的kitty

### b. 使用伪代码解释流程

from account的链表中remove kitty；
from account的count - 1；
to account的链表中append kitty；
to account的count + 1；

## 3. 设计如何实现简单的交易功能，用户可以给自己小猫定价，然后其他人可以出钱购买

### a. 分析新增的 stroage 数据结构

新增map：kitty -》 BalanceOf<T>

### b. 使用伪代码解释流程

增加setPrice（如lesson3的为claim设置price）

增减bugKitty（如lesson3的购买claim）
