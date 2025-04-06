import Markdown from 'react-markdown';
import token from '../assets/token.md?raw';

import { HeaderChallenge } from '@/components/HeaderChallenge';

export const Challenge = () => {
  return (
    <div className="mt-8">
      <HeaderChallenge />

      <div className="prose ml-[25%] mt-8">
        <Markdown>{token}</Markdown>
      </div>
    </div>
  );
};
