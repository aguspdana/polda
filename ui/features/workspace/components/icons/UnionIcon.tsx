export function UnionIcon({ color }: { color: string }) {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect y="1" width="14" height="6" fill={color}/>
      <rect y="9" width="14" height="6" fill={color}/>
    </svg>
  );
}