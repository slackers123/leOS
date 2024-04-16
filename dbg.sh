KSRC_DIR="leos-kernel"
ARCH_TARGET="x86-leos-kernel"
K_NAME="leos-kernel"
BOOT_DIR="leos-boot"
IMG_NAME="kernel.img"

MKBOOTIMG=$BOOT_DIR/mkbootimg

# compile the kernel
cd $KSRC_DIR
cargo b 
if [ $? -ne 0 ]; then
  echo "Cargo error."
  exit 1
fi
cd ..

# move the kernel to initrd
mv target/$ARCH_TARGET/debug/$K_NAME $BOOT_DIR/initrd/

if [ $? -ne 0 ]; then
  echo "error moving kernel"
  exit 1
fi

# make the acutal boot image
$MKBOOTIMG $BOOT_DIR/leos.json $BOOT_DIR/$IMG_NAME

if [ $? -ne 0 ]; then
  echo "error creating boot image"
  exit 1
fi

qemu-system-x86_64 $BOOT_DIR/$IMG_NAME
