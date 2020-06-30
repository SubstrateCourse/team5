import React, { useEffect, useState } from 'react';
import { Button, Form, Input, Grid } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import { blake2AsHex } from '@polkadot/util-crypto';
import { hexToString } from '@polkadot/util';

function Main (props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');

  // The currently stored value
  const [digest, setDigest] = useState('');
  const [owner, setOwner] = useState('');
  const [dest, setDest] = useState('');
  const [queryAddress, setQueryAddress] = useState('');
  const [note, setNote] = useState('');
  const [blockNumber, setBlockNumber] = useState(0);
  const [UserProofs, setUserProofs] = useState('');

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
    const fileReader = new FileReader();

    const bufferToDigest = () => {
      const content = Array.from(new Uint8Array(fileReader.result))
        .map((b) => b.toString().padStart(2, '0'))
        .join('');

      const hash = blake2AsHex(content, 256);
      setDigest(hash);
    };

    fileReader.onloadend = bufferToDigest;

    fileReader.readAsArrayBuffer(file);
  };

  const handleTransfer = (_, data) => setDest(data.value);
  const handleNote = (_, data) => setNote(data.value);
  const handleQueryAddress = (_, data) => setQueryAddress(data.value);

  const queryUserProof = () => {
    api.query.poeModule.accountProofs(queryAddress, (proofs) => {
      const proofsWithDetail = [];
      proofs.forEach(proof => api.query.poeModule.proofsWithNote(proof, (proofWithDetail) => {
        proofsWithDetail.push({
          digest: proof,
          timestamp: proofWithDetail[2],
          note: hexToString(proofWithDetail[3].toHuman())
        });
      }));
      setUserProofs(JSON.stringify(proofsWithDetail));
    });
  };

  return (
    <Grid.Column width={8}>
      <h1>Proof of Existence Module</h1>
      <Form>
        <Form.Field>
          <Input
            type="file"
            id="file"
            label="Your File"
            onChange={ (e) => handleFileChosen(e.target.files[0]) }
          />
        </Form.Field>

        <Form.Field>
          <Input
            fluid
            label='note'
            type='text'
            placeholder='note'
            state='note'
            onChange={handleNote}
          />
        </Form.Field>

        <Form.Field>
          <TxButton
            accountPair={accountPair}
            label='create claim'
            type='SIGNED-TX'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaimWithNote',
              inputParams: [digest, note],
              paramFields: [true]
            }}
          />
          <TxButton
            accountPair={accountPair}
            label='revoke claim'
            type='SIGNED-TX'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'poeModule',
              callable: 'revokeClaim',
              inputParams: [digest],
              paramFields: [true]
            }}
          />
        </Form.Field>
        <Form.Field>
          <Input
            fluid
            label='To'
            type='text'
            placeholder='address'
            state='addressTo'
            onChange={handleTransfer}
          />
        </Form.Field>
        <Form.Field>
          <TxButton
            accountPair={accountPair}
            label='transfer claim'
            type='SIGNED-TX'
            setStatus={setStatus}
            attrs={{
              palletRpc: 'poeModule',
              callable: 'transferClaim',
              inputParams: [digest, dest],
              paramFields: [true]
            }}
          />
        </Form.Field>
      </Form>
      <div style={{ overflowWrap: 'break-word' }}>{status}</div>
      <div>{`Claim info, owner: ${owner}, blockNumber: ${blockNumber}, note: ${note}`}</div>
      <Form>
        <Form.Field>
          <Input
            fluid
            state='queryAccount'
            type='text'
            placeholder='address'
            label='Address to Query'
            onChange={handleQueryAddress}
          />
        </Form.Field>
        <Form.Field>
          <Button
            color='blue'
            basic
            onClick={queryUserProof}
          >
            Query User Doc
          </Button>
        </Form.Field>
        <div>{UserProofs}</div>
      </Form>
    </Grid.Column>
  );
}

export default function TemplateModule (props) {
  const { api } = useSubstrate();
  return (api.query.templateModule && api.query.templateModule.something
    ? <Main {...props} /> : null);
}
