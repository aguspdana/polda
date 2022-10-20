export function ComputeIcon({ color }: { color: string }) {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect x="11.5" y="9.5" width="1" height="1" fill={color} stroke={color}/>
      <rect x="11.5" y="14.5" width="1" height="1" fill={color} stroke={color}/>
      <rect x="9" y="12" width="6" height="1" fill={color}/>
      <rect x="1" y="3" width="6" height="2" fill={color}/>
      <path d="M9 3H15V5H9V3Z" fill={color}/>
      <path d="M1.5 10L6.5 15" stroke={color} strokeWidth="2"/>
      <path d="M1.5 15L6.5 10" stroke={color} strokeWidth="2"/>
      <rect x="3" y="1" width="2" height="6" fill={color}/>
    </svg>
  )
}