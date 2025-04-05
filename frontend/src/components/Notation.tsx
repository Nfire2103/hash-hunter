export const Notation = ({ rating }: { rating: number }) => {
  const stars = Array.from({ length: 5 }, (_, index) => index + 1);

  return (
    <div className="flex items-center ml-[-2px]">
      {stars.map((star) => (
        <svg
          key={star}
          xmlns="http://www.w3.org/2000/svg"
          fill={star <= rating ? 'currentColor' : 'none'}
          viewBox="0 0 32 32"
          strokeWidth={2}
          stroke="currentColor"
          className="h-4 w-6 text-yellow-500"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l2.122 6.564a1 1 0 00.95.69h6.905c.969 0 1.371 1.24.588 1.81l-5.588 4.06a1 1 0 00-.364 1.118l2.122 6.564c.3.921-.755 1.688-1.54 1.118l-5.588-4.06a1 1 0 00-1.176 0l-5.588 4.06c-.784.57-1.838-.197-1.54-1.118l2.122-6.564a1 1 0 00-.364-1.118l-5.588-4.06c-.783-.57-.38-1.81.588-1.81h6.905a1 1 0 00.95-.69l2.122-6.564z"
          />
        </svg>
      ))}
    </div>
  );
};
