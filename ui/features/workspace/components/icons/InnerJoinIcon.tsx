export function InnerJoinIcon({ color }: { color : string }) {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
      <circle cx="6" cy="8" r="5.5" stroke={color}/>
      <path fillRule="evenodd" clipRule="evenodd" d="M8 13.6586C10.3304 12.8349 12 10.6125 12 8.00002C12 5.38758 10.3304 3.1651 8 2.34143C5.66962 3.1651 4 5.38758 4 8.00002C4 10.6125 5.66962 12.8349 8 13.6586Z" fill={color}/>
      <circle cx="10" cy="8" r="5.5" stroke={color}/>
    </svg>
  );
}