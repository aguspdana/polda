export default function AddColumnIcon({ color }: { color: string }) {
  return (
    <svg width="25" height="24" viewBox="0 0 25 24" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect x="4.5" y="6" width="4" height="12" fill={color}/>
      <rect x="10.5" y="6" width="4" height="12" fill={color}/>
      <rect x="16.5" y="11" width="6" height="2" fill={color}/>
      <rect x="18.5" y="9" width="2" height="6" fill={color}/>
    </svg>
  );
}