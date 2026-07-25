import { Input, InputGroup } from "@chakra-ui/react";
import { useRef, useState } from "react";
import { LuSearch } from "react-icons/lu";

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
    <InputGroup startElement={<LuSearch />}>
      <Input
        placeholder={placeholder}
        value={value}
        onChange={e => handleValueChange(e.target.value)}
      />
    </ InputGroup>
  );
}

function sanitizeValue(value: string): string {
  const val = value.trim();
  return val.length >= 3 ? val : '';
}
