import { Outlet } from 'react-router';

import { Navbar } from '@/components/Navbar';

export const Layout = () => {
  return (
    <div className="py-6 px-[2.5rem]">
      <Navbar />
      <Outlet />
    </div>
  );
};
