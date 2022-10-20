export function BinsIcon({ color }: { color: string }) {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect y="6" width="4" height="8" fill={color}/>
      <rect x="6" y="2" width="4" height="12" fill={color}/>
      <rect x="12" y="10" width="4" height="4" fill={color}/>
    </svg>
  );
}