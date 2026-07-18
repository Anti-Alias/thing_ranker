import { Input } from "@chakra-ui/react";
import { useRef, useState } from "react";

const SEARCH_DELAY_MS = 500;

interface SearchInputProps {
  placeholder?: string,
  onSearch: (value: string) => void,
}

export default function SearchInput({ placeholder, onSearch }: SearchInputProps) {
  const [value, setValue] = useState('');
  const timeoutIdRef = useRef<number | null>(null);

  const handleValueChange = (nextValue: string) => {
    const lastVal = sanitizeValue(value);
    const currentVal = sanitizeValue(nextValue);
    setValue(nextValue);
    if (lastVal != currentVal) {
      if (timeoutIdRef.current) {
        clearTimeout(timeoutIdRef.current);
      }
      timeoutIdRef.current = setTimeout(() => {
        onSearch(currentVal);
      }, SEARCH_DELAY_MS);
    }
  };

  return (
    <Input
      placeholder={placeholder}
      value={value}
      onChange={e => handleValueChange(e.target.value)}
    />
  );
}

function sanitizeValue(value: string): string {
  return value.length >= 3 ? value : '';
}
