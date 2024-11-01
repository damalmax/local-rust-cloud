aws iam create-virtual-mfa-device \
    --virtual-mfa-device-name TestUser \
    --outfile /tmp/QRCode.png \
    --bootstrap-method QRCodePNG \
    --endpoint-url http://localhost:4502/iam/

aws iam create-role --role-name Test-Role \
    --assume-role-policy-document file://create_user__permissions_boundary.json \
    --tags '{"Key": "Department", "Value": "Accounting"}' '{"Key": "Location", "Value": "3Seattle"}' \
    --endpoint-url http://localhost:4502/iam/

aws iam list-policies --endpoint-url http://localhost:4502/iam/

aws iam attach-role-policy \
    --policy-arn "arn:aws:iam::000000000001:policy/my-policy2" \
    --role-name Test-Role \
    --endpoint-url http://localhost:4502/iam/

aws iam create-role \
    --role-name Test-Role \
    --assume-role-policy-document file://create_user__permissions_boundary.json \
    --tags '{"Key": "Department", "Value": "Accounting"}' '{"Key": "Location", "Value": "3Seattle"}' \
    --endpoint-url http://localhost:4502/iam/

aws iam create-policy \
    --policy-name my-policy3 \
    --tags '{"Key": "Department", "Value": "Accounting"}' '{"Key": "Location", "Value": "Seattle"}' \
    --policy-document file://create_user__permissions_boundary.json \
    --endpoint-url http://localhost:4502/iam/

aws iam list-groups --endpoint-url http://localhost:4502/iam/

aws iam create-group \
    --group-name Admins4 \
    --endpoint-url http://localhost:4502/iam/

