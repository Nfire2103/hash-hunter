import { ChevronDown } from 'lucide-react';

import { ChallengeCard } from '@/components/ChallengeCard';
import { GradientButton } from '@/components/GradientButton';
import { Input } from '@/components/ui/input';

export const ListChallenges = () => {
  return (
    <div className="flex flex-col items-center space-y-12">
      <div className="flex flex-col items-center space-y-4 pt-16">
        <h1 className="text-8xl">Pick your course.</h1>
        <h1 className="text-8xl">
          <span className="text-accent">{'{Solve}'}</span> it!
        </h1>
      </div>

      <div className="flex space-x-4">
        <Input placeholder="Search course" className="rounded-none w-[24rem]" />

        <GradientButton>
          <div className="flex items-center space-x-2 group-hover:text-accent transition-colors">
            <span>Choose Difficulty</span>
            <ChevronDown />
          </div>
        </GradientButton>

        <GradientButton>
          <div className="flex items-center space-x-2 group-hover:text-accent transition-colors">
            <span>Choose Language</span>
            <ChevronDown />
          </div>
        </GradientButton>

        <GradientButton>
          <div className="flex items-center space-x-2 group-hover:text-accent transition-colors">
            <span>Sort by</span>
            <ChevronDown />
          </div>
        </GradientButton>
      </div>

      <div className="w-full grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-8">
        <ChallengeCard />
        <ChallengeCard />
        <ChallengeCard />
        <ChallengeCard />
      </div>
    </div>
  );
};
