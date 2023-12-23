#[doc = "Register `dig_clk_cfg0` reader"]
pub struct R(crate::R<DIG_CLK_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIG_CLK_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIG_CLK_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIG_CLK_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dig_clk_cfg0` writer"]
pub struct W(crate::W<DIG_CLK_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIG_CLK_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DIG_CLK_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIG_CLK_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dig_32k_div` reader - "]
pub type DIG_32K_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `dig_32k_div` writer - "]
pub type DIG_32K_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIG_CLK_CFG0_SPEC, u16, u16, 11, O>;
#[doc = "Field `reserved_11` reader - "]
pub type RESERVED_11_R = crate::BitReader<bool>;
#[doc = "Field `dig_32k_en` reader - "]
pub type DIG_32K_EN_R = crate::BitReader<bool>;
#[doc = "Field `dig_32k_en` writer - "]
pub type DIG_32K_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIG_CLK_CFG0_SPEC, bool, O>;
#[doc = "Field `dig_32k_comp` reader - "]
pub type DIG_32K_COMP_R = crate::BitReader<bool>;
#[doc = "Field `dig_32k_comp` writer - "]
pub type DIG_32K_COMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIG_CLK_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_14_15` reader - "]
pub type RESERVED_14_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dig_512k_div` reader - "]
pub type DIG_512K_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dig_512k_div` writer - "]
pub type DIG_512K_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIG_CLK_CFG0_SPEC, u8, u8, 7, O>;
#[doc = "Field `reserved_23` reader - "]
pub type RESERVED_23_R = crate::BitReader<bool>;
#[doc = "Field `dig_512k_en` reader - "]
pub type DIG_512K_EN_R = crate::BitReader<bool>;
#[doc = "Field `dig_512k_en` writer - "]
pub type DIG_512K_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIG_CLK_CFG0_SPEC, bool, O>;
#[doc = "Field `dig_512k_comp` reader - "]
pub type DIG_512K_COMP_R = crate::BitReader<bool>;
#[doc = "Field `dig_512k_comp` writer - "]
pub type DIG_512K_COMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIG_CLK_CFG0_SPEC, bool, O>;
#[doc = "Field `reserved_26_27` reader - "]
pub type RESERVED_26_27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dig_clk_src_sel` reader - "]
pub type DIG_CLK_SRC_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dig_clk_src_sel` writer - "]
pub type DIG_CLK_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DIG_CLK_CFG0_SPEC, u8, u8, 2, O>;
#[doc = "Field `reserved_30` reader - "]
pub type RESERVED_30_R = crate::BitReader<bool>;
#[doc = "Field `reg_en_platform_wakeup` reader - "]
pub type REG_EN_PLATFORM_WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `reg_en_platform_wakeup` writer - "]
pub type REG_EN_PLATFORM_WAKEUP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DIG_CLK_CFG0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn dig_32k_div(&self) -> DIG_32K_DIV_R {
        DIG_32K_DIV_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reserved_11(&self) -> RESERVED_11_R {
        RESERVED_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dig_32k_en(&self) -> DIG_32K_EN_R {
        DIG_32K_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dig_32k_comp(&self) -> DIG_32K_COMP_R {
        DIG_32K_COMP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn reserved_14_15(&self) -> RESERVED_14_15_R {
        RESERVED_14_15_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn dig_512k_div(&self) -> DIG_512K_DIV_R {
        DIG_512K_DIV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reserved_23(&self) -> RESERVED_23_R {
        RESERVED_23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dig_512k_en(&self) -> DIG_512K_EN_R {
        DIG_512K_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dig_512k_comp(&self) -> DIG_512K_COMP_R {
        DIG_512K_COMP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn reserved_26_27(&self) -> RESERVED_26_27_R {
        RESERVED_26_27_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dig_clk_src_sel(&self) -> DIG_CLK_SRC_SEL_R {
        DIG_CLK_SRC_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn reserved_30(&self) -> RESERVED_30_R {
        RESERVED_30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_en_platform_wakeup(&self) -> REG_EN_PLATFORM_WAKEUP_R {
        REG_EN_PLATFORM_WAKEUP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn dig_32k_div(&mut self) -> DIG_32K_DIV_W<0> {
        DIG_32K_DIV_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn dig_32k_en(&mut self) -> DIG_32K_EN_W<12> {
        DIG_32K_EN_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn dig_32k_comp(&mut self) -> DIG_32K_COMP_W<13> {
        DIG_32K_COMP_W::new(self)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    #[must_use]
    pub fn dig_512k_div(&mut self) -> DIG_512K_DIV_W<16> {
        DIG_512K_DIV_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn dig_512k_en(&mut self) -> DIG_512K_EN_W<24> {
        DIG_512K_EN_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn dig_512k_comp(&mut self) -> DIG_512K_COMP_W<25> {
        DIG_512K_COMP_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn dig_clk_src_sel(&mut self) -> DIG_CLK_SRC_SEL_W<28> {
        DIG_CLK_SRC_SEL_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_en_platform_wakeup(&mut self) -> REG_EN_PLATFORM_WAKEUP_W<31> {
        REG_EN_PLATFORM_WAKEUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dig_clk_cfg0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dig_clk_cfg0](index.html) module"]
pub struct DIG_CLK_CFG0_SPEC;
impl crate::RegisterSpec for DIG_CLK_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dig_clk_cfg0::R](R) reader structure"]
impl crate::Readable for DIG_CLK_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dig_clk_cfg0::W](W) writer structure"]
impl crate::Writable for DIG_CLK_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dig_clk_cfg0 to value 0x033e_13e8"]
impl crate::Resettable for DIG_CLK_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x033e_13e8;
}
