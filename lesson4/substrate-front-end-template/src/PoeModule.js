import React, { useEffect, useState } from 'react';
import { Form, Input, Grid } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import {
  Dropdown
} from 'semantic-ui-react';
import { blake2AsHex } from '@polkadot/util-crypto'

function Main (props) {
  const { api } = useSubstrate();
  const { keyring } = useSubstrate();
  const { accountSelected } = props;
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');

  const [digest, setDigest] = useState('');
  const [owner, setOwner] = useState('');
  const [blockNumber, setBlockNumber] = useState(0);
  const [dest, setDest] = useState('');
  const [memo, setMemo] = useState('');

  // Get the list of accounts we possess the private key for
  const keyringOptions = keyring.getPairs().map(account => ({
    key: account.address,
    value: account.address,
    text: account.meta.name.toUpperCase(),
    icon: 'user'
  }));

  useEffect(() => {
    let unsubscribe;
    api.query.poeModule.proofs(digest, (result) => {
      setOwner(result[0].toString());
      setBlockNumber(result[1].toNumber());
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [digest, api.query.poeModule]);

  const handleFileChosen = (file) => {
    console.log(file)
    let fileReader = new FileReader();

    const bufferToDigest = () => {
      const content = Array.from(new Uint8Array(fileReader.result))
        .map((b) => b.toString(16).padStart(2, '0'))
        .join(' ')

      const hash = blake2AsHex(content, 256)
      setDigest(hash)
    }

    fileReader.onloadend = bufferToDigest;
    fileReader.readAsArrayBuffer(file);

  }
  const onDestChange = address => {
    // Update state with new account address
    setDest(address);
  };
  const onMemoChange = memo => {
    // Update state with new account address
    setMemo(memo);
  };
  return (
    <Grid.Column width={8}>
      <h1>Proof of existance Module</h1>
      <Form>
        <Form.Field>
          <Input
            type='file'
            id='file'
            label='proof file'
            onChange={ (e) => handleFileChosen(e.target.files[0])}
          />
        </Form.Field>
        <Form.Field>
          <Input
            fluid
            label='memo'
            type='text'
            placeholder='memo'
            state=''
            onChange={(e)=>{onMemoChange(e.target.value)}}
          />
        </Form.Field>
        <Form.Field>
          <TxButton
            accountPair={accountPair}
            label='Crate Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaim',
              inputParams: [digest],
              paramFields: [true]
            }}
          />
          <TxButton
            accountPair={accountPair}
            label='Revoke Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'revokeClaim',
              inputParams: [digest],
              paramFields: [true]
            }}
          />
          <TxButton
            accountPair={accountPair}
            label='Crate Claim with Memo'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaimWithMemo',
              inputParams: [digest,memo],
              paramFields: [true,true]
            }}
          />
        </Form.Field>
        <Form.Field>
          <Dropdown
              search
              selection
              clearable
              placeholder='Select an account to transfer your claim'
              options={keyringOptions}
              onChange={(_, dropdown) => {
                onDestChange(dropdown.value);
              }}
              value={accountSelected}
            />
          <TxButton
            accountPair={accountPair}
            label='Transfer Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'transferClaim',
              inputParams: [digest, dest],
              paramFields: [true, dest]
            }}
          />
        </Form.Field>
        <div style={{ overflowWrap: 'break-word' }}>{status}</div>
        <div>{status}</div>
        <div>{`Claim info: owner ${owner}, blockNumber ${blockNumber}`}</div>
      </Form>
    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
