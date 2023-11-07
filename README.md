# Local Rust Cloud DEV
The project is created to help with serverless apps testing on local environment.

# Services supported

## AWS
| Service Name | Binary name               | Depends On           |
|--------------|---------------------------|----------------------|
| STS          | `local_rust_cloud_sts_rs` |                      |

### Services implementation coverage

#### 2. STS
- [x] assume_role
- [ ] assume_role_with_web_identity

# How to contribute
All services are implemented using [Rust language](https://www.rust-lang.org/).


# Windows Build prerequisites

Guide Used: https://gist.github.com/zeljic/d8b542788b225b1bcb5fce169ee28c55

### How to build SQLite3 .lib file on Windows 10

1. Download source from [source](https://www.sqlite.org/download.html) (https://www.sqlite.org/download.html)
	For example: [source](https://www.sqlite.org/2022/sqlite-amalgamation-3390300.zip) `https://www.sqlite.org/2022/sqlite-amalgamation-3390300.zip`
2. Download binary from [binary](https://www.sqlite.org/download.html)
	For example: [binary](https://www.sqlite.org/2022/sqlite-dll-win64-x64-3390300.zip) `https://www.sqlite.org/2022/sqlite-dll-win64-x64-3390300.zip`

3. Extract both archives to the same directory

4. Open **Developer Command Prompt for VS 2017** by typing *Developer Command* in Windows Search

5. Go to directory where you've extracted **source code** and **binary** files (via opened cmd)
6. Run
	```lib /DEF:sqlite3.def /OUT:sqlite3.lib /MACHINE:x64```
