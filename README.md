# Exoneum
Permissionless blockchain based on Exonum framework 

# Services

1. Service for add new user to the network. User must be validator 
    - generate pub_key for user and store userdata on disk(folder db)
    - send notification(new user's ip:port) to other validators. After that update node configuration and add new validator to pull
        - if other validator offline
        - if validator online
2. Service for transfer value to other users
3. Service for distributed file storage
4. Service for exchange other cryptocurrencies