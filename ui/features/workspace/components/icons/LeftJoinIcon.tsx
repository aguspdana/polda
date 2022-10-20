export default function LeftJoinIcon({ color }: { color: string }) {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
      <circle cx="6" cy="8" r="5.5" fill={color}/>
      <circle cx="10" cy="8" r="5.5" stroke={color}/>
    </svg>
  );
}