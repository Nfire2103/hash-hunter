import { Button } from '@/components/ui/button';

export const GradientButton = ({ children }: { children: React.ReactNode }) => {
  return (
    <div className="group relative">
      <div className="absolute -inset-1 rounded-none bg-accent opacity-30 blur transition duration-500 group-hover:opacity-60"></div>
      <Button className="relative rounded-none bg-background py-2 px-4 text-primary hover:bg-background">
        {children}
      </Button>
    </div>
  );
};
