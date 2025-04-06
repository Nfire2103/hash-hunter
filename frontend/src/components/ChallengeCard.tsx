import { Link } from 'react-router';

import { Notation } from '@/components/Notation';
import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';

type ChallengeCardProps = {
  image: string;
  title: string;
  description: string;
  rating: number;
};

export const ChallengeCard = ({
  image,
  title,
  description,
  rating,
}: ChallengeCardProps) => {
  return (
    <Card className="w-full rounded-none bg-secondary">
      <CardHeader>
        <img src={image} className="rounded-none object-cover h-48" />
      </CardHeader>

      <CardContent className="space-y-6">
        <div className="space-y-1 h-16">
          <CardTitle className="leading-5">{title}</CardTitle>
          <CardDescription>{description}</CardDescription>
        </div>

        <div className="space-y-1">
          <Notation rating={rating} />
          <CardDescription>{rating}/5 (Based on user votes)</CardDescription>
        </div>
      </CardContent>

      <CardFooter className="mt-auto">
        <Button className="w-full rounded-none bg-background hover:bg-background text-primary hover:text-accent">
          <Link to="/challenge/162ba840-099e-11f0-a9cc-7b4d2cd6a4c5">
            Solve now
          </Link>
        </Button>
      </CardFooter>
    </Card>
  );
};
