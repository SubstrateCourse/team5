## 第三课作业  PoE 2

课程里会给出参考资料，大家一定要自己敲一遍**代码**！

注：

1. 提交源代码，运行`cargo test`的测试结果截图，前端UI的截图；
2. 测试应覆盖所有的业务逻辑，如不同的错误场景，以及一切正常的情况下，检查存储的数据是不是预期的那样。
3. 附加题不是必答的，但可以酌情加分。
4. 代码修改在本目录 substrate-node-template 和 substrate-front-end-template 的程序文件里。

第一题：编写存证模块的单元测试代码，包括：

* 创建存证的测试用例；
* 撤销存证的测试用例；
* 转移存证的测试用例；

**cargo test截图**
![test](./test.png)

第二题：编写存证模块的UI，包括

* 创建存证的UI
* 删除存证的UI
* 转移存证的UI

**部署和浏览器**
![ui-terminal](./ui-terminal.png)

![ui](./ui.png)

**创建存证的UI**
![ui-create-claim-1](./ui-create-claim-1.png)

![ui-create-claim-2](./ui-create-claim-2.png)

**删除存证的UI**
![ui-revoke-claim-1](./ui-revoke-claim-1.png)

![ui-revoke-claim-2](./ui-revoke-claim-2.png)

**转移存证的UI**
![ui-transfer-claim-1](./ui-transfer-claim-1.png)

![ui-transfer-claim-2](./ui-transfer-claim-2.png)

第三题（附加题）：实现购买存证的功能代码：

* 用户A为自己的某个存证记录设置价格；
* 用户B可以以一定的价格购买某个存证，当出价高于用户A设置的价格时，则以用户A设定的价格将费用从用户B转移到用户A，再将该存证进行转移。如果出价低于用户A的价格时，则不进行转移，返回错误。

**Alice创建存证**
![buy-claim-01](./buy-claim-01.png)

**查看存证信息**
![buy-claim-02](./buy-claim-02.png)

**Alice设置存证价格99**
![buy-claim-03](./buy-claim-03.png)

**查询存证金额**
![buy-claim-04](./buy-claim-04.png)

**Bob使用88购买存证失败**
![buy-claim-05](./buy-claim-05.png)

**Bob使用100购买存证成功**
![buy-claim-06](./buy-claim-06.png)

**查看存证信息**
![buy-claim-07](./buy-claim-07.png)
