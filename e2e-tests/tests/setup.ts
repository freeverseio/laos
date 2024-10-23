// const awaitBlockProduction = async (nodeUrl: string) => {
// 	const api = await ApiPromise.create({
// 		provider: new HttpProvider(nodeUrl),
// 		noInitWarn: true,
// 	});

// 	await api.isReady;

// 	let counter = 3;
// 	let blocksProducing = false;
// 	while (!blocksProducing) {
// 		const { number } = await api.rpc.chain.getHeader();

// 		if (number.toNumber() > 0) {
// 			blocksProducing = true;
// 		}
// 		await delay(1000);

// 		counter += 1;
// 	}

// 	await api.disconnect().then(() => {});
// };

// const awaitEpochChange = async () => {
// 	const apiRelaychain = await ApiPromise.create({
// 		provider: new HttpProvider(RELAYCHAIN_NODE_URL),
// 		noInitWarn: true,
// 	});

// 	await apiRelaychain.isReady;
// 	// Fetch the current epoch index and cast it to u64
// 	const currentEpochIndexCodec = await apiRelaychain.query.babe.epochIndex();
// 	const currentEpochIndex = (currentEpochIndexCodec as u64).toNumber();

// 	let counter = 1;
// 	let changedEpoch = false;

// 	while (!changedEpoch) {
// 		// Fetch the latest epoch index and cast it to u64
// 		const epochIndexCodec = await apiRelaychain.query.babe.epochIndex();
// 		const epochIndex = (epochIndexCodec as u64).toNumber();

// 		// Compare the numerical values
// 		if (epochIndex > currentEpochIndex + 1) {
// 			changedEpoch = true;
// 		}

// 		await delay(1000);
// 		counter += 1;
// 	}

// 	console.log(`Epoch has changed after ${counter} seconds.`);
// };

// const openChannel = async (api: ApiPromise, sender: number, recipient: number) => {
// 	console.log("[HRMP] Opening channel between ", sender, " and ", recipient);
// 	const maxCapacity = 8;
// 	const maxMessageSize = 1048576;
// 	const keyring = new Keyring({ type: "sr25519" });
// 	const sudo = keyring.addFromUri("//Alice");

// 	const tx = api.tx.hrmp.forceOpenHrmpChannel(sender, recipient, maxCapacity, maxMessageSize);
// 	api.tx.sudo
// 		.sudo(tx)
// 		.signAndSend(sudo, () => {})
// 		.catch((error: any) => {
// 			console.log("transaction failed", error);
// 		});

// 	while ((await isChannelOpen(api, sender, recipient)) == false) {
// 		console.log("Waiting till channel is open..");
// 		await delay(1000);
// 	}
// };

// before(async function () {
// 	console.log("[RELAY_CHAIN] Waiting for block production...");
// 	const apiRelaychain = await ApiPromise.create({
// 		provider: new HttpProvider(RELAYCHAIN_NODE_URL),
// 		noInitWarn: true,
// 	});
// 	await awaitBlockChange(apiRelaychain);

// 	console.log("[RELAY_CHAIN] Opening channels..."); // See: https://github.com/paritytech/polkadot-sdk/pull/1616
// 	await sendOpenHrmpChannelTxs(apiRelaychain);
	
// 	console.log("[ASSET_HUB] Waiting for block production...");
// 	const apiAssetHub = await ApiPromise.create({
// 		provider: new HttpProvider(ASSET_HUB_NODE_URL),
// 		noInitWarn: true,
// 	});
// 	await awaitBlockChange(apiAssetHub);
	
// 	console.log("[LAOS] Waiting for block production...");
// 	const apiLaos = await ApiPromise.create({
// 		provider: new HttpProvider(LAOS_NODE_URL),
// 		noInitWarn: true,
// 	});
// 	await awaitBlockChange(apiLaos);

// 	while (
// 		(await isChannelOpen(apiRelaychain, LAOS_PARA_ID, ASSET_HUB_PARA_ID)) == false ||
// 		(await isChannelOpen(apiRelaychain, ASSET_HUB_PARA_ID, LAOS_PARA_ID)) == false
// 	) {
// 		await awaitBlockChange(apiRelaychain);
// 	}
// 	await apiRelaychain.disconnect();
// 	await apiAssetHub.disconnect();
// 	await apiLaos.disconnect();
// });
