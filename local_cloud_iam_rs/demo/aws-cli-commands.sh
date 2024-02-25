aws iam create-virtual-mfa-device \
    --virtual-mfa-device-name TestUser \
    --outfile /tmp/QRCode.png \
    --bootstrap-method QRCodePNG \
    --endpoint-url http://localhost:4502/iam/
