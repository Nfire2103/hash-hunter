import { render, screen } from '@testing-library/react';
import { Home } from '@/pages/Home';

describe('Home component', () => {
  test('renders heading and button', () => {
    render(<Home />);

    // Check the main heading
    const heading = screen.getByRole('heading', {
      name: /welcome to hash hunter/i,
    });
    expect(heading).toBeInTheDocument();

    // Check the button
    const button = screen.getByRole('button', { name: /get started/i });
    expect(button).toBeInTheDocument();

    // Optional: check the footer text
    const footerText = screen.getByText(
      new RegExp(
        `© ${new Date().getFullYear()} Hash Hunter. All rights reserved.`,
        'i',
      ),
    );
    expect(footerText).toBeInTheDocument();
  });
});
