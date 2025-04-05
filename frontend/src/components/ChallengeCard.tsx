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

export const ChallengeCard = ({
  image,
  rating,
}: {
  image: string;
  rating: number;
}) => {
  return (
    <Card className="w-full rounded-none bg-secondary">
      <CardHeader>
        <img src={image} className="rounded-none" />
      </CardHeader>

      <CardContent className="space-y-6">
        <div className="space-y-1">
          <CardTitle>How to create a token</CardTitle>
          <CardDescription>Solana</CardDescription>
        </div>
        <div className="space-y-1">
          <Notation rating={rating} />
          <CardDescription>4.2/5 (Based on user votes)</CardDescription>
        </div>
      </CardContent>

      <CardFooter>
        <Button className="w-full rounded-none bg-background hover:bg-background text-primary hover:text-accent ">
          Solve now
        </Button>
      </CardFooter>
    </Card>
  );
};
