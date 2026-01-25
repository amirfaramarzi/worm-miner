.PHONY: download_params

download_params:
	mkdir -p ~/.worm-miner
	echo "Downloading parameter files..."
	cd ~/.worm-miner && wget $(WGET_ARGS) -c https://github.com/worm-privacy/trusted-setup/releases/download/final/final.tar.gz.aa
	cd ~/.worm-miner && wget $(WGET_ARGS) -c https://github.com/worm-privacy/trusted-setup/releases/download/final/final.tar.gz.ab
	cd ~/.worm-miner && wget $(WGET_ARGS) -c https://github.com/worm-privacy/trusted-setup/releases/download/final/final.tar.gz.ac
	cd ~/.worm-miner && wget $(WGET_ARGS) -c https://github.com/worm-privacy/trusted-setup/releases/download/final/final.tar.gz.ad
	cd ~/.worm-miner && wget $(WGET_ARGS) -c https://github.com/worm-privacy/trusted-setup/releases/download/final/final.tar.gz.ae

	echo "Extracting parameter files..."
	cat ~/.worm-miner/final.tar.gz.a* > ~/.worm-miner/params.tar.gz
	cd ~/.worm-miner && tar xzf params.tar.gz

	echo "Downloading dat files..."
	cd ~/.worm-miner && wget $(WGET_ARGS) -c https://github.com/worm-privacy/trusted-setup/releases/download/circuit_data/proof_of_burn.dat
	cd ~/.worm-miner && wget $(WGET_ARGS) -c https://github.com/worm-privacy/trusted-setup/releases/download/circuit_data/spend.dat

	rm -rf ~/.worm-miner/final.tar.gz.a*
	rm -rf ~/.worm-miner/params.tar.gz
	echo "Done!"
