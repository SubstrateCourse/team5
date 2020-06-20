import React, { useState, useEffect } from 'react';
import { List, Grid, Button, Input, Modal } from 'semantic-ui-react';

// Pre-built Substrate front-end utilities for connecting to a node
// and making a transaction.
import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
// Polkadot-JS utilities for hashing data.
import { blake2AsHex } from '@polkadot/util-crypto';

export default function Main(props) {
  const { api, keyring } = useSubstrate();
	const { accountPair } = props;
	// The transaction submission status
	const [ status, setStatus ] = useState('');

	// The currently stored value

	const [ proofList, setProofList ] = useState([]);
	const [ proofNumber, setProofNumber ] = useState([]);
	const [ itemRenders, setItemRenders ] = useState(0);

	const keyringOptions = keyring.getPairs().map((account) => ({
		key: account.address,
		value: account.address,
		text: account.meta.name.toUpperCase(),
		icon: 'user',
	}));

	const getKeyringName = (addr) => {
		let name = '';
		keyringOptions.map((value) => {
			if (name === '' && value.key === addr) {
				name = value.text;
			}
		});
		if (name !== '') return name;
		return addr;
	};

	useEffect(
		() => {
			let unsubscribe;
			if (accountPair) {
				api.query.poe
					.ownedProofs(accountPair.address, (arr) => {
						if (arr.isNone) {
							setProofList([]);
						} else {
							let list = arr.toJSON();

							setProofList(list);
						}
					})
					.then((unsub) => {
						unsubscribe = unsub;
					})
					.catch(console.error);
			}
			return () => unsubscribe && unsubscribe();
		},
		[ api.query.poe, accountPair],
	);

	useEffect(
		() => {
			getItemRenders(proofList);
		},
		[ accountPair, proofList],
	);

	const getProofDetails = async (proof) => {
		let obj = await api.query.poe.proofs(proof);
		return !obj.isNone ? obj.unwrap().toJSON() : null;
	};

	const getItemRenders = async (proofList) => {
		let list = [];
		for (var i = 0; i < proofList.length; i++) {
      let proof = proofList[i];
      let proof_info = await getProofDetails(proof);
      list.push(
				<List.Item key={i}>
					<br />
					<List.Content
						style={{
							padding: 20,
							backgroundColor: '#fff',
							fontSize: 18,
							color: '#333',
						}}
					>
						<div style={{ fontSize: 18 }}>
              <br />编号:{i}
              <br />proof_hash:{getKeyringName(proof)}：
							<br />comment:{proof_info.comment}
              <br />price:{proof_info.price}
              <br />created_on:{proof_info.created_on}
              <br />claim_block:{proof_info.claim_block}
						</div>
						<br />
					
					</List.Content>
				</List.Item>,
			);
		}
    setItemRenders(list);
		setProofNumber(list.length);
	};

	return (
		<Grid.Column
			style={{
				backgroundColor: '#eceefa',
				padding: 20,
			}}
		>
			<h1>My Proofs{'(' + proofNumber + ')'}</h1>
			<List>{itemRenders}</List>
			<div style={{ overflowWrap: 'break-word' }}>{status}</div>
		</Grid.Column>
  );
}




