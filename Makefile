make:
	cd daemon && cargo update
	cd daemon && cargo build -r 

install:
	mkdir -p /etc/macbook-cpu-fan/conf/ 
	cp -rv ./conf/* /etc/macbook-cpu-fan/conf/ 
	echo "std.json" > /etc/macbook-cpu-fan/conf.txt 
	cp ./daemon/target/release/daemon /etc/macbook-cpu-fan/ 
	cp ./macbook-cpu-fan.service /etc/macbook-cpu-fan/ 
	cp ./cli/mbcf /usr/local/bin
	systemctl enable /etc/macbook-cpu-fan/macbook-cpu-fan.service 
	systemctl start macbook-cpu-fan.service

uninstall:
	systemctl stop macbook-cpu-fan.service
	systemctl disable macbook-cpu-fan.service
	rm -rf /etc/macbook-cpu-fan/ 
	rm -rf ./daemon/target 
