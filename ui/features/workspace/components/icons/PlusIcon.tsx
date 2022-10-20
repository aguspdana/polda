export default function PlusIcon({ color }: { color: string }) {
  return (
    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
      <rect x="11" y="6" width="2" height="12" fill={color}/>
      <rect x="6" y="11" width="12" height="2" fill={color}/>
    </svg>
  )
}