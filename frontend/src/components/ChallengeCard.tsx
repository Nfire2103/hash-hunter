import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';

export const ChallengeCard = () => {
  return (
    <Card className="w-full">
      <CardHeader>Image</CardHeader>
      <CardContent>
        <CardTitle>How to create a token</CardTitle>
        <CardDescription>Solana</CardDescription>
      </CardContent>
    </Card>
  );
};
