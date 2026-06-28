import os

# --- BU 3 DEGISKENI KENDINIZE GORE DUZENLEYIN ---
GITHUB_USERNAME = "Onfephost"
GITHUB_TOKEN = "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"  # eski token'i iptal edip yenisini buraya yazin
REPO_NAME = "TestOnfe"
ZIP_PATH = "/sdcard/Download/Acode-main-fixed-3.zip"
ZIP_NAME = os.path.basename(ZIP_PATH)
# --------------------------------------------------

commands = [
    "pkg update -y",
    "pkg install -y git zip unzip",
    "termux-setup-storage",
    f"cp {ZIP_PATH} ~/",
    f"cd ~ && unzip -o {ZIP_NAME}",
    
    "cd ~/Acode-main && git init",
    "cd ~/Acode-main && git add .",
    'cd ~/Acode-main && git commit -m "Fix all files access permission"',
    "cd ~/Acode-main && git branch -M main",
    f"cd ~/Acode-main && git remote add origin https://{GITHUB_USERNAME}:{GITHUB_TOKEN}@github.com/{GITHUB_USERNAME}/{REPO_NAME}.git",
    "cd ~/Acode-main && git push -u origin main",
]

for cmd in commands:
    print(f"\n>>> Calistiriliyor: {cmd}\n")
    exit_code = os.system(cmd)
    if exit_code != 0:
        print(f"\n!!! HATA: '{cmd}' komutu basarisiz oldu (kod: {exit_code}). Durduruluyor.\n")
        break
else:
    print("\n✅ Tum islemler tamamlandi. GitHub repo'nuzu kontrol edin.\n")
