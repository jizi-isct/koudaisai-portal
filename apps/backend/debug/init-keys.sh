openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:2048 -out private-key.pem
openssl pkey -in private-key.pem -pubout -out public-key.pem