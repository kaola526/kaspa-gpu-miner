VERSION=0.1.1

.PHONY: build
build:
	cargo build --release --all

# F2JLJG9sxEgE
# sudo ./kaspa-miner --mining-address kaspa:qzn4fltcsh30n22f6zszvuy9pkzjnmz97dcvm740wd5l98dqw94q6s820ggvg --kaspad-address stratum+tcp://192.168.11.75:5555
# scp ./target/release/libkaspacuda.so fil@192.168.11.51:/home/fil/kaspa/
# scp ./target/release/libkaspaopencl.so fil@192.168.11.51:/home/fil/kaspa/
.PHONY: dev
dev:
	scp ./target/release/kaspa-miner fil@192.168.11.51:/home/fil/kaspa/