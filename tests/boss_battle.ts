import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenBattle } from "../target/types/token_battle";
import { expect } from "chai";

describe("Boss Battle Game", () => {
  // Configurar el provider
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TokenBattle as Program<TokenBattle>;
  
  // Constantes del juego (deben coincidir con constants.rs)
  const MAX_ENERGY_PLAYER = 100;
  const MAX_ENERGY_ENEMY = 100;
  const SEED = Buffer.from("Boss");

  describe("Initialize", () => {
    it("Debe inicializar correctamente la cuenta del juego", async () => {
      // Usar la wallet del provider
      const player = provider.wallet.publicKey;
      
      // Derivar el PDA
      const [gameDataPDA, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [SEED, player.toBuffer()],
        program.programId
      );

      console.log("🎮 Player:", player.toString());
      console.log("📦 Game Data PDA:", gameDataPDA.toString());
      console.log("🔢 Bump:", bump);

      // Verificar que la cuenta no existe antes
      let accountInfo = await provider.connection.getAccountInfo(gameDataPDA);
      if (accountInfo !== null) {
        console.log("⚠️  La cuenta ya existe. Limpia el validador con: solana-test-validator --reset");
        throw new Error("La cuenta del juego ya existe. Reinicia el validador local.");
      }

      // Llamar a initialize
      const tx = await program.methods
        .initialize()
        .accounts({
          owner: player,
        })
        .rpc();

      console.log("✅ Transacción de inicialización:", tx);

      // Verificar el estado inicial
      const gameData = await program.account.gameData.fetch(gameDataPDA);
      
      expect(gameData.owner.toString()).to.equal(player.toString());
      expect(gameData.playerEnergy.toNumber()).to.equal(MAX_ENERGY_PLAYER);
      expect(gameData.enemyEnergy.toNumber()).to.equal(MAX_ENERGY_ENEMY);

      console.log("✅ Estado inicial verificado:");
      console.log("   👤 Player Energy:", gameData.playerEnergy.toNumber());
      console.log("   👹 Enemy Energy:", gameData.enemyEnergy.toNumber());
    });
  });

  describe("Attack", () => {
    const player = anchor.AnchorProvider.env().wallet.publicKey;
    const [gameDataPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [SEED, player.toBuffer()],
      program.programId
    );

    it("Debe reducir la energía del jugador y del enemigo", async () => {
      // Obtener estado inicial
      let gameData = await program.account.gameData.fetch(gameDataPDA);
      const initialPlayerEnergy = gameData.playerEnergy.toNumber();
      const initialEnemyEnergy = gameData.enemyEnergy.toNumber();

      console.log("⚔️  Estado antes del ataque:");
      console.log("   👤 Player:", initialPlayerEnergy);
      console.log("   👹 Enemy:", initialEnemyEnergy);

      // Realizar ataque
      const tx = await program.methods
        .attack()
        .accounts({
          owner: player,
        })
        .rpc();

      console.log("✅ Ataque ejecutado:", tx);

      // Verificar estado después del ataque
      gameData = await program.account.gameData.fetch(gameDataPDA);
      const finalPlayerEnergy = gameData.playerEnergy.toNumber();
      const finalEnemyEnergy = gameData.enemyEnergy.toNumber();

      console.log("⚔️  Estado después del ataque:");
      console.log("   👤 Player:", finalPlayerEnergy, `(${initialPlayerEnergy - finalPlayerEnergy} de daño recibido)`);
      console.log("   👹 Enemy:", finalEnemyEnergy, `(${initialEnemyEnergy - finalEnemyEnergy} de daño infligido)`);

      // Verificar que las energías se redujeron
      expect(finalPlayerEnergy).to.be.lessThanOrEqual(initialPlayerEnergy);
      expect(finalEnemyEnergy).to.be.lessThanOrEqual(initialEnemyEnergy);
    });

    it("Debe continuar atacando hasta que alguien muera", async () => {
      let gameData = await program.account.gameData.fetch(gameDataPDA);
      let roundCount = 0;
      const MAX_ROUNDS = 50; // Límite de seguridad para evitar loops infinitos

      console.log("\n🎯 Iniciando batalla completa...\n");

      while (
        gameData.playerEnergy.toNumber() > 0 && 
        gameData.enemyEnergy.toNumber() > 0 &&
        roundCount < MAX_ROUNDS
      ) {
        roundCount++;
        
        await program.methods
          .attack()
          .accounts({
            owner: player,
          })
          .rpc();

        gameData = await program.account.gameData.fetch(gameDataPDA);
        
        console.log(`   Round ${roundCount}:`);
        console.log(`      👤 Player: ${gameData.playerEnergy.toNumber()}`);
        console.log(`      👹 Enemy: ${gameData.enemyEnergy.toNumber()}`);
      }

      // Verificar que el combate terminó
      const playerDead = gameData.playerEnergy.toNumber() === 0;
      const enemyDead = gameData.enemyEnergy.toNumber() === 0;

      expect(playerDead || enemyDead).to.be.true;

      if (enemyDead) {
        console.log("\n🎉 ¡Victoria! El enemigo ha sido derrotado.");
      } else {
        console.log("\n💀 Derrota. El jugador ha muerto.");
      }

      console.log(`⚔️  Total de rounds: ${roundCount}`);
    });
  });

  describe("Respawn", () => {
    const player = anchor.AnchorProvider.env().wallet.publicKey;
    const [gameDataPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [SEED, player.toBuffer()],
      program.programId
    );

    it("Debe reiniciar las energías a sus valores máximos", async () => {
      // Obtener estado antes del respawn
      let gameData = await program.account.gameData.fetch(gameDataPDA);
      
      console.log("💀 Estado antes del respawn:");
      console.log("   👤 Player:", gameData.playerEnergy.toNumber());
      console.log("   👹 Enemy:", gameData.enemyEnergy.toNumber());

      // Ejecutar respawn
      const tx = await program.methods
        .respawn()
        .accounts({
          owner: player,
        })
        .rpc();

      console.log("✅ Respawn ejecutado:", tx);

      // Verificar que las energías se restauraron
      gameData = await program.account.gameData.fetch(gameDataPDA);

      console.log("✨ Estado después del respawn:");
      console.log("   👤 Player:", gameData.playerEnergy.toNumber());
      console.log("   👹 Enemy:", gameData.enemyEnergy.toNumber());

      expect(gameData.playerEnergy.toNumber()).to.equal(MAX_ENERGY_PLAYER);
      expect(gameData.enemyEnergy.toNumber()).to.equal(MAX_ENERGY_ENEMY);
    });
  });

  describe("Flujo completo del juego", () => {
    it("Debe permitir jugar varias partidas seguidas", async () => {
      const player = provider.wallet.publicKey;
      const [gameDataPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [SEED, player.toBuffer()],
        program.programId
      );

      const GAMES_TO_PLAY = 3;

      for (let game = 1; game <= GAMES_TO_PLAY; game++) {
        console.log(`\n🎮 ====== PARTIDA ${game}/${GAMES_TO_PLAY} ======`);

        // Respawn (excepto la primera vez si ya se jugó antes)
        if (game > 1) {
          await program.methods
            .respawn()
            .accounts({ owner: player })
            .rpc();
          console.log("✨ Partida reiniciada");
        }

        let gameData = await program.account.gameData.fetch(gameDataPDA);
        let roundCount = 0;

        // Jugar hasta que alguien muera
        while (
          gameData.playerEnergy.toNumber() > 0 && 
          gameData.enemyEnergy.toNumber() > 0
        ) {
          roundCount++;
          
          await program.methods
            .attack()
            .accounts({ owner: player })
            .rpc();

          gameData = await program.account.gameData.fetch(gameDataPDA);
        }

        if (gameData.enemyEnergy.toNumber() === 0) {
          console.log(`🎉 Victoria en ${roundCount} rounds`);
        } else {
          console.log(`💀 Derrota en ${roundCount} rounds`);
        }
      }

      console.log("\n✅ Todas las partidas completadas exitosamente");
    });

    it("Register victory using events", async () => {
      const player = provider.wallet.publicKey;
      const [gameDataPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [SEED, player.toBuffer()],
        program.programId
      );
      //Prepare the listener
      let victoryEvent = null;
      const listener = program.addEventListener("bossDefeated", (event) => {
        victoryEvent = event;
      });

      //Initialize game (respawn)
      await program.methods
        .respawn()
        .accounts({ owner: player })
        .rpc();
      console.log("✨ Partida reiniciada");

      // Attack until someone dies
      let gameData = await program.account.gameData.fetch(gameDataPDA);
      let roundCount = 0;
      while (
        gameData.playerEnergy.toNumber() > 0 && 
        gameData.enemyEnergy.toNumber() > 0
      ) {
        roundCount++;
        
        await program.methods
          .attack()
          .accounts({ owner: player })
          .rpc();

        gameData = await program.account.gameData.fetch(gameDataPDA);
      }

      // Verify event
      if (victoryEvent) {
        console.log("🏆 Ranking actualizado para:", victoryEvent.player.toString());
        console.log("💖 Energía restante:", victoryEvent.player_energy.toNumber());
        console.log("💖 Timestamp:", victoryEvent.timestamp.toNumber());      
      }
      await program.removeEventListener(listener);
    });
  });

  
});