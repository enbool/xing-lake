# xing-lake
## 1. Introduction
This project is a config center that created by Golang

## 2. Features
- [x] Support multi data source
- [x] Support multi data type
- [x] Support multi data format
- [x] Support multi data storage
- [x] Support multi data sync
## 3. Quick Start
### 3.1 Install
```shell
go get -u https://github.com/enbool/xing-lake
```
#### Install Goland dependencies
#### Install Rust dependencies
#### Install Docker
```shell
#### Docker
Starter Repository for Docker: https://github.com/enbool/xing-lake/lake
Run Script:
```shell
docker run -d -p 8080:8080 --name xing-lake enbool/xing-lake
```
```shell
### 3.2 Usage
#### 3.2.1 Create a config center
```go
ConfigServer := xing_lake.NewConfigServer()
```
#### 3.2.2 Add a data source
```go
ConfigServer.AddDataSource("mysql", "mysql://root:123456@localhost:3306/xing_lake")
```
#### 3.2.3 Add a data type
```go
ConfigServer.AddDataType("json", "json")
```
#### 3.2.4 Add a data format
```go
ConfigServer.AddDataFormat("json", "json")
```
#### 3.2.5 Create a config client
```go
ConfigClient := xing_lake.NewConfigClient()
```
#### 3.2.5 Connect to the config center
```go
ConfigClient.Connect("http://localhost:8080")
```
#### 3.2.6 Get a config
```go
ConfigClient.GetConfig("mysql", "json", "json")
```
#### 3.2.7 Set a config
```go
ConfigClient.SetConfig("mysql", "json", "json", "{'name':'xing-lake'}")
```
#### 3.2.8 Delete a config
```go
ConfigClient.DeleteConfig("mysql", "json", "json")
```
## 4. Contributing
We encourage you to contribute to xing-lake! Please check out the [Contributing to xing-lake guide #TODO]
## 5. License
xing-lake is under the [Apache License, Version 2.0](LICENSE)
