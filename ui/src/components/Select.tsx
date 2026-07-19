import { Select as ChakraSelect, Portal, type ListCollection, } from "@chakra-ui/react";

interface ValueChangeDetails<T> {
  value: string[];
  items: T[];
}

interface SelectProps {
  collection: ListCollection<Elem>;
  width?: number;
  value?: string[];
  onValueChange?: ((details: ValueChangeDetails<Elem>) => void);
}

interface Elem {
  value: string;
  label: string;
}

/**
 * A helper component that simplifies making a Chakra Select element
 */
export default function Select(props: SelectProps) {
  return (
    <ChakraSelect.Root {...props}>
      <ChakraSelect.Control>
        <ChakraSelect.Trigger>
          <ChakraSelect.ValueText />
        </ChakraSelect.Trigger>
        <ChakraSelect.IndicatorGroup>
          <ChakraSelect.Indicator />
        </ChakraSelect.IndicatorGroup>
      </ChakraSelect.Control>
      <Portal>
        <ChakraSelect.Positioner>
          <ChakraSelect.Content>
            {props.collection.items.map(item => (
              <ChakraSelect.Item item={item} key={item.value}>
                {item.label}
                <ChakraSelect.ItemIndicator />
              </ChakraSelect.Item>
            ))}
          </ChakraSelect.Content>
        </ChakraSelect.Positioner>
      </Portal>
    </ChakraSelect.Root>
  );
}
