from lwk import *

mnemonic = Mnemonic("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about")
network = Network.testnet()
assert(str(network) == "SequentiaTestnet")

client = network.default_electrum_client()
client.ping()

signer = Signer(mnemonic, network)
desc = signer.wpkh_slip77_descriptor()

assert(str(desc) == "ct(slip77(9c8e4f05c7711a98c838be228bcb84924d4570ca53f35fa1c793e58841d47023),elwpkh([73c5da0a/84'/1'/0']tpubDC8msFGeGuwnKG9Upg7DM2b4DaRqg3CUZa5g8v2SRQ6K4NSkxUgd7HsL2XVWbVm39yBA4LAxysQAm397zwQSQoQgewGiYZqrA9DsP4zbQ1M/<0;1>/*))#2e4n992d")

wollet = Wollet(network, desc, datadir=None)

receiver_pubkey = "028af0e1d6ff3bb43c8161eb73ff91759a83dea9b9cbce9b60f09c8cc5cf880d0d"
owner_pubkey = "02e6aaef17549e6a375d0dd305b618a2d58168caadc9fd5e59f2b2b84368f73adf"
seed_hash = "ed80f84ab619dadac421242053e794cf30781d65d6ce6ff509f75badbb688b3e"


htlc = wollet.create_htlc(receiver_pubkey, owner_pubkey, 10, seed_hash)

