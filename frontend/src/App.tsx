import { Route, Routes } from 'react-router';
import './App.css';

import { Layout } from '@/components/Layout';
import { Challenge } from '@/pages/Challenge';
import { Home } from '@/pages/Home';
import { ListChallenges } from '@/pages/ListChallenges';
import { CreateChallenge } from '@/pages/CreateChallenge';

function App() {
  return (
    <>
      <Routes>
        <Route index element={<Home />} />

        <Route element={<Layout />}>
          <Route path="/challenge">
            <Route index element={<ListChallenges />} />
            <Route path=":id" element={<Challenge />} />
          </Route>
          <Route path="/create-challenge" element={<CreateChallenge />} />
        </Route>
      </Routes>
    </>
  );
}

export default App;
