import solanaLogo from '../assets/solanaLogo.png';

export const HeaderChallenge = () => {
  return (
    <div className="flex items-center justify-center">
      <div className="w-3xl flex space-x-4">
        <img src={solanaLogo} className="size-16" />
        <div className="flex flex-col justify-center">
          <h1 className="text-xl font-medium">Solana</h1>
          <h2>20 min read · 1 correction · 1 day ago</h2>
        </div>
      </div>
    </div>
  );
};
