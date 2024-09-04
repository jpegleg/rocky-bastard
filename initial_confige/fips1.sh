fips-mode-setup --check || yum install crypto-policies-scripts -y
fips-mode-setup --enable 
reboot
