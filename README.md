# test_siblink
Тестовое задание для SibLink

Часть 1. Запуск тестовой сети hyperledger fabric.  

1. curl -sSL https://bit.ly/2ysbOFE | bash -s
2. cd fabric-samples/test-network
3. ./network.sh up createChannel
4. export FABRIC_CFG_PATH=${HOME}/test_fabric/fabric-samples/config  
export CORE_PEER_ADDRESS="localhost:7051"  export CORE_PEER_LOCALMSPID="Org1MSP"  
export CORE_PEER_MSPCONFIGPATH="${HOME}/test_fabric/fabric-samples/test-network/organizations/peerOrganizations/org1.example.com/users/Admin@org1.example.com/msp"  
export CORE_PEER_TLS_ENABLED="true"  
export CORE_PEER_TLS_ROOTCERT_FILE="${HOME}/test_fabric/fabric-samples/test-network/organizations/peerOrganizations/org1.example.com/peers/peer0.org1.example.com/tls/ca.crt"
5. ./network.sh deployCC -ccl javascript
6. ../bin/peer chaincode invoke -o localhost:7050 --ordererTLSHostnameOverride orderer.example.com --tls --cafile 
7. ${PWD}/organizations/ordererOrganizations/example.com/orderers/orderer.example.com/msp/tlscacerts/tlsca.example.com-cert.pem -C mychannel -n basic --peerAddresses localhost:7051 --tlsRootCertFiles ${PWD}/organizations/peerOrganizations/org1.example.com/peers/peer0.org1.example.com/tls/ca.crt --peerAddresses localhost:9051 --tlsRootCertFiles ${PWD}/organizations/peerOrganizations/org2.example.com/peers/peer0.org2.example.com/tls/ca.crt -c '{"function":"InitLedger","Args":[]}'


Часть 2. Запуск приложения.  

1. git clone https://github.com/karnaukhovlad/test_siblink.git
2. cd ./test_siblink
3. cargo install
4. cargo run

Для того, что бы получить все ассеты нужен get 0.0.0.0:8384/
Для того, что бы получить конкретный ассет get 0.0.0.0:8384/{asset_id}. Пример: 0.0.0.0:8384/asset1
