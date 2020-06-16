import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, Button, Label } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import { blake2AsHex } from '@polkadot/util-crypto';
import AccountSelector from './AccountSelector';
import { web3FromSource } from '@polkadot/extension-dapp';

function Main(props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');
  const [digest, setDigest] = useState('');
  const [owner, setOwner] = useState('');
  const [blockNumber, setBlockNumber] = useState(0);
  const [accountAddress, setAccountAddress] = useState(null);
  const [anotherAddress, setAnotherAddress] = useState(null);
  const [comment, setComment] = useState('');
  const [docDetails, setDocDetails] = useState(null);

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
    let fileReader = new FileReader();

    const bufferToDigest = () => {
      const content = Array.from(new Uint8Array(fileReader.result))
        .map((b) => b.toString(16).padStart(2, '0'))
        .join('');

      const hash = blake2AsHex(content, 256);
      setDigest(hash);
    }

    fileReader.onloadend = bufferToDigest;
    fileReader.readAsArrayBuffer(file);
  }

  const getFromAcct = async () => {
    console.log(accountPair);

    const {
      address,
      meta: { source, isInjected }
    } = accountPair;
    let fromAcct;

    // signer is from Polkadot-js browser extension
    if (isInjected) {
      const injected = await web3FromSource(source);
      fromAcct = address;
      api.setSigner(injected.signer);
    } else {
      fromAcct = accountPair;
    }

    return fromAcct;
  };

  const submitDocInfo = async (hash, comment) => {
    const statusCallback = ({ status }) => status.isFinalized ? setStatus('区块已最终确认!') : setStatus(`当前状态：${status.type}`);

    const account = await getFromAcct();
    api.tx.poeModule.createClaim(hash, comment).signAndSend(account, statusCallback);
  }

  const bin2String = (array) => {
    let result = "";
    for (let i = 0; i < array.length; i++) {
      result += String.fromCharCode(parseInt(array[i]));
    }
    return result;
  }

  const getDocInfoFromAddr = async (addr) => {
    const rtn = await api.query.poeModule.accountDocs(addr);
    let details = [];
    for (let i = 0; i < rtn.length; i++) {
      details.push({
        docHash: blake2AsHex(rtn[i][0], 256),
        createdOn: new Date(parseInt(rtn[i][1])),
        comment: bin2String(rtn[i][2]),
      })
    }
    console.log(details);
    setDocDetails(JSON.stringify(details));
  }

  return (
    <Grid.Column width={8}>
      <h1>Proof of Existence Module</h1>
      <Form>
        <Form.Field>
          <Input
            type='file'
            id='file'
            label='Your File'
            onChange={(e) => handleFileChosen(e.target.files[0])}
          />
          <Input
            type='comment'
            id='comment'
            label='File Comment'
            maxLength='256'
            value={comment}
            onChange={e => setComment(e.target.value)}
          />
          <Button
            type='submit'
            id='submit'
            onClick={() => submitDocInfo(digest, comment)}>
            Submit File Info
              </Button>
        </Form.Field>

        <Form.Field>
          {/* we don't use AccountSelector here, get from input string directly */}
          <Input
            type='address'
            id='address'
            label='Search Address'
            value={anotherAddress}
            onChange={e => setAnotherAddress(e.target.value)}
          />
          <Button
            type='search'
            id='search'
            onClick={() => getDocInfoFromAddr(anotherAddress)}>
            Searcha Account Claims
          </Button>
          <p />
          <Label
            type='docs'
            id='docs'>
            {docDetails}
          </Label>
        </Form.Field>

        <Form.Field>
          <TxButton
            accountPair={accountPair}
            label='Create Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaim',
              inputParams: [digest, comment], paramFields: [true]
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
        </Form.Field>

        <Form.Field>
          <TxButton
            accountPair={accountPair}
            label='Transfer Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'transferClaim',
              inputParams: [digest, accountAddress],
              paramFields: [true]
            }}
          />

          <AccountSelector setAccountAddress={setAccountAddress} />
        </Form.Field>

        <div>{status}</div>
        <div>{`Claim info, owner: ${owner}, blockNumber: ${blockNumber}`}</div>
      </Form>
    </Grid.Column>
  );
}

export default function PoeModule(props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
