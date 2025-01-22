import { vars } from "@/styles/theme";
import { Overlay, useMantineColorScheme } from "@mantine/core";

function BlankTopBar() {
  const { colorScheme } = useMantineColorScheme();
  return (
    <>
      <Overlay
        bg={colorScheme === "dark" ? vars.colors.dark[7] : vars.colors.white}
        data-tauri-drag-region
      />
    </>
  );
}

export default BlankTopBar;
