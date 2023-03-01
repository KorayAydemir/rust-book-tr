# Hata Yönetimi

Hatalar; yazılımda hayatın bir parçasıdır, bu yüzden Rust bir şeylerin yanlış
gittiği durumları yönetmek için bir takım özelliklere sahiptir. Çoğu durumda
Rust, hata çıkması olasılığının farkında olmanızı ve kodunuzu derlemeden önce
harekete geçmenizi gerektirir. Bu gereksinim, daha kodunuzu tüketime sunmadan
önce hataları keşfedip uygun şekilde yönetmenizi sağlayarak programınızı daha
sağlam yapar!

Rust hataları iki ana gruba ayırır: *kurtarılabilir* ve *kurtarılamaz* hatalar.
Kurtarılabilir bir hata için, örneğin *dosya bulunamadı* hatası, büyük olasılıkla
sadece sorunu kullanıcıya bildirip işlemi yeniden denemek isteriz. Kurtarılamaz
hatalar her zaman sistem açığının işaretçisidir, mesela bir dizinin sonundan ötede
bir yere erişmeye çalışmak gibi, ve bu yüzden programı anında durdurmak isteriz.

Çoğu dil bu iki tip hata arasında ayrım yapmaz ve ikini de aynı şekilde,
"istisnalar" gibi teknikler kullanarak yönetir. Rust'ta istisnalar yoktur. Onun
yerine, kurtarılabilir hatalar için `Result<T,E>` türüne ve program bir hata ile
karşılaştığında programı çalıştırmayı duduran `panic!` macrosuna sahiptir. Bu bölüm,
öncelikle "panic!"i çağırmayı kapsar ve sonra `Result<T, E>` değerlerini döndürmek
hakkında konuşur. Ek olarak, bir hatadan kurtarmaya çalışmaya veya programın
çalışmasını durdurmaya karar verirken göz önünde bulundurulmasi gerekenleri inceleyeceğiz.
