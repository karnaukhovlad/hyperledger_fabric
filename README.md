# Тестовое задание для SibLink

### Часть 1. Запуск тестовой сети hyperledger fabric.  
***
```
curl -sSL https://bit.ly/2ysbOFE | bash -s
```
```
cd fabric-samples/test-network
```
```
./network.sh up createChannel
```
```
export FABRIC_CFG_PATH=${HOME}/test_fabric/fabric-samples/config  
export CORE_PEER_ADDRESS="localhost:7051"  
export CORE_PEER_LOCALMSPID="Org1MSP"  
export CORE_PEER_MSPCONFIGPATH="${HOME}/test_fabric/fabric-samples/test-network/organizations/peerOrganizations/org1.example.com/users/Admin@org1.example.com/msp"  
export CORE_PEER_TLS_ENABLED="true"  
export CORE_PEER_TLS_ROOTCERT_FILE="${HOME}/test_fabric/fabric-samples/test-network/organizations/peerOrganizations/org1.example.com/peers/peer0.org1.example.com/tls/ca.crt"
```
```
./network.sh deployCC -ccl javascript
```
```
../bin/peer chaincode invoke -o localhost:7050 --ordererTLSHostnameOverride orderer.example.com \
--tls --cafile ${PWD}/organizations/ordererOrganizations/example.com/orderers/orderer.example.com/msp/tlscacerts/tlsca.example.com-cert.pem \
-C mychannel -n basic --peerAddresses localhost:7051 \
--tlsRootCertFiles ${PWD}/organizations/peerOrganizations/org1.example.com/peers/peer0.org1.example.com/tls/ca.crt --peerAddresses localhost:9051 --tlsRootCertFiles ${PWD}/organizations/peerOrganizations/org2.example.com/peers/peer0.org2.example.com/tls/ca.crt -c '{"function":"InitLedger","Args":[]}'
```

### Часть 2. Запуск приложения.  
***
```
git clone https://github.com/karnaukhovlad/test_siblink.git && cd ./test_siblink && cargo install && cargo run
```
Для того, что бы получить все ассеты необходимо сделать get запрос 0.0.0.0:8384/
Для того, что бы получить конкретный ассет get 0.0.0.0:8384/{asset_id}. Пример: 0.0.0.0:8384/asset1

### Часть 3. Результаты нагрузочного тестирования ab -n 100000 -c 100 http://0.0.0.0:8384/
***
```
Server Software:        
Server Hostname:        0.0.0.0
Server Port:            8384

Document Path:          /
Document Length:        828 bytes

Concurrency Level:      100
Time taken for tests:   1122.269 seconds
Complete requests:      100000
Failed requests:        0
Total transferred:      93000000 bytes
HTML transferred:       82800000 bytes
Requests per second:    89.11 [#/sec] (mean)
Time per request:       1122.269 [ms] (mean)
Time per request:       11.223 [ms] (mean, across all concurrent requests)
Transfer rate:          80.93 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.1      0       5
Processing:    83 1122 678.0    951    7199
Waiting:       83 1122 678.0    951    7199
Total:         83 1122 678.0    951    7199

Percentage of the requests served within a certain time (ms)
  50%    951
  66%   1221
  75%   1447
  80%   1607
  90%   2063
  95%   2460
  98%   2943
  99%   3289
 100%   7199 (longest request)
```
####Скриншот htop, показывающий нагрузку на потоки при тестировании. 
![Распределение нагрузки](/home/vlad/Pictures/Screenshots/Screenshot from 2020-08-18 19-01-03.png "Распределение нагрузки")